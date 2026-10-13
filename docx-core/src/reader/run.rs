#![allow(clippy::single_match)]

use std::io::Read;
use std::str::FromStr;

use super::Run;

use crate::escape::replace_escaped;
use crate::types::BreakType;
use crate::{reader::*, FieldCharType};

#[derive(PartialEq, Debug)]
enum TextState {
    Idle,
    Text,
    Delete,
}

fn read_field_char(attributes: &[OwnedAttribute]) -> Result<FieldChar, ReaderError> {
    let mut t: Option<FieldCharType> = None;
    let mut dirty = false;
    for a in attributes {
        let local_name = &a.name.local_name;
        match local_name.as_str() {
            "fldCharType" => {
                if let Ok(ty) = FieldCharType::from_str(&a.value) {
                    t = Some(ty);
                }
            }
            "dirty" => {
                dirty = !is_false(&a.value);
            }
            _ => {}
        }
    }

    if let Some(t) = t {
        let mut f = FieldChar::new(t);
        if dirty {
            f = f.dirty();
        }
        Ok(f)
    } else {
        Err(ReaderError::XMLReadError)
    }
}

// Namespace URIs identify run vocabularies; prefixes are arbitrary and scoped.
// Keep undeclared conventional prefixes for legacy XML-fragment callers.
fn has_namespace(name: &OwnedName, namespaces: &[&str], legacy_prefix: &str) -> bool {
    match name.namespace.as_deref() {
        Some(namespace) => namespaces.contains(&namespace),
        None => name.prefix.as_deref() == Some(legacy_prefix),
    }
}

pub(super) fn is_word_name(name: &OwnedName) -> bool {
    has_namespace(
        name,
        &[
            "http://schemas.openxmlformats.org/wordprocessingml/2006/main",
            "http://purl.oclc.org/ooxml/wordprocessingml/main",
        ],
        "w",
    )
}

impl ElementReader for Run {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        _attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut run = Run::new();
        let mut text_state = TextState::Idle;
        loop {
            let e = r.next_event();
            match e {
                Ok(XmlEvent::StartElement {
                    attributes, name, ..
                }) => {
                    match () {
                        _ if is_word_name(&name) => {
                            let e = XMLElement::from_str(&name.local_name).unwrap();

                            ignore::ignore_element(e.clone(), XMLElement::RunPropertyChange, r);

                            match e {
                                XMLElement::Tab => {
                                    run = run.add_tab();
                                }
                                XMLElement::PTab => {
                                    if let Ok(v) = PositionalTab::read(r, &attributes) {
                                        run = run.add_ptab(v);
                                    }
                                }
                                XMLElement::Sym => {
                                    if let Some(font) = read(&attributes, "font") {
                                        if let Some(char) = read(&attributes, "char") {
                                            let sym = Sym::new(font, char);
                                            run = run.add_sym(sym);
                                        }
                                    }
                                }
                                XMLElement::RunProperty => {
                                    let p = RunProperty::read(r, &attributes)?;
                                    run = run.set_property(p);
                                }
                                XMLElement::Text => text_state = TextState::Text,
                                XMLElement::DeleteText => text_state = TextState::Delete,
                                XMLElement::Break => {
                                    if let Some(attribute) = attributes.iter().find(|attribute| {
                                        attribute.name.local_name == "type"
                                            && (is_word_name(&attribute.name)
                                                || (attribute.name.namespace.is_none()
                                                    && attribute.name.prefix.is_none()))
                                    }) {
                                        run = run.add_break(BreakType::from_str(&attribute.value)?)
                                    } else {
                                        run = run.add_break(BreakType::TextWrapping)
                                    }
                                }
                                XMLElement::CarriageReturn => {
                                    run = run.add_carriage_return();
                                }
                                XMLElement::Drawing => {
                                    if let Ok(drawing) = Drawing::read(r, &attributes) {
                                        run = run.add_drawing(drawing);
                                    }
                                }
                                XMLElement::FieldChar => {
                                    if let Ok(f) = read_field_char(&attributes) {
                                        run.children.push(RunChild::FieldChar(f));
                                    }
                                }
                                XMLElement::InstrText => loop {
                                    let e = r.next_event();
                                    match e {
                                        Ok(XmlEvent::Characters(c)) => {
                                            run.children.push(RunChild::InstrTextString(c));
                                            break;
                                        }
                                        Ok(XmlEvent::EndElement { name, .. })
                                            if is_word_name(&name) =>
                                        {
                                            let e = XMLElement::from_str(&name.local_name).unwrap();
                                            match e {
                                                XMLElement::Run => {
                                                    return Ok(run);
                                                }
                                                _ => {}
                                            }
                                        }
                                        Ok(XmlEvent::EndDocument) => {
                                            return Err(ReaderError::XMLReadError)
                                        }
                                        Err(_) => return Err(ReaderError::XMLReadError),
                                        _ => {}
                                    }
                                },
                                _ => {}
                            }
                        }
                        _ if has_namespace(
                            &name,
                            &["http://schemas.openxmlformats.org/markup-compatibility/2006"],
                            "mc",
                        ) =>
                        {
                            let e = McXMLElement::from_str(&name.local_name).unwrap();
                            match e {
                                McXMLElement::Fallback => {
                                    let _ = McFallback::read(r, &attributes)?;
                                }
                                _ => {}
                            }
                        }
                        _ if has_namespace(&name, &["urn:schemas-microsoft-com:vml"], "v") => {
                            let e = VXMLElement::from_str(&name.local_name).unwrap();
                            match e {
                                // Experimental For now support only imageData in shape
                                VXMLElement::Shape => {
                                    if let Ok(shape) = Shape::read(r, &attributes) {
                                        run.children.push(RunChild::Shape(Box::new(shape)));
                                    }
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    };
                }
                Ok(XmlEvent::Characters(c)) => match text_state {
                    TextState::Delete => {
                        run = run.add_delete_text_without_escape(replace_escaped(&c));
                    }
                    TextState::Text => {
                        run = run.add_text_without_escape(replace_escaped(&c));
                    }
                    _ => {}
                },
                Ok(XmlEvent::Whitespace(c)) => match text_state {
                    TextState::Delete => {
                        run = run.add_delete_text_without_escape(replace_escaped(&c));
                    }
                    TextState::Text => {
                        run = run.add_text_without_escape(replace_escaped(&c));
                    }
                    _ => {}
                },
                Ok(XmlEvent::EndElement { name, .. }) if is_word_name(&name) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    match e {
                        XMLElement::Run => {
                            return Ok(run);
                        }
                        XMLElement::DeleteText | XMLElement::Text => text_state = TextState::Idle,
                        _ => {}
                    }
                }
                Ok(XmlEvent::EndDocument) => return Err(ReaderError::XMLReadError),
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn read_word_containers_ignore_math_and_foreign_runs() {
        for container in ["p", "hyperlink", "ins", "del", "moveFrom", "moveTo", "sdt"] {
            for prefix in ["w", "text"] {
                let read_container =
                    |xml: &str| -> Result<serde_json::Value, ReaderError> {
                        let mut reader = EventReader::new(xml.as_bytes());
                        match container {
                            "p" => Paragraph::read(&mut reader, &[])
                                .map(|v| serde_json::to_value(v).unwrap()),
                            "hyperlink" => Hyperlink::read(&mut reader, &[])
                                .map(|v| serde_json::to_value(v).unwrap()),
                            "ins" => Insert::read(&mut reader, &[])
                                .map(|v| serde_json::to_value(v).unwrap()),
                            "del" => Delete::read(&mut reader, &[])
                                .map(|v| serde_json::to_value(v).unwrap()),
                            "moveFrom" => MoveFrom::read(&mut reader, &[])
                                .map(|v| serde_json::to_value(v).unwrap()),
                            "moveTo" => MoveTo::read(&mut reader, &[])
                                .map(|v| serde_json::to_value(v).unwrap()),
                            "sdt" => StructuredDataTag::read(&mut reader, &[])
                                .map(|v| serde_json::to_value(v).unwrap()),
                            _ => unreachable!(),
                        }
                    };
                let before =
                    format!("<{prefix}:r><{prefix}:t>Before equation</{prefix}:t></{prefix}:r>");
                let after =
                    format!("<{prefix}:r><{prefix}:t>After equation</{prefix}:t></{prefix}:r>");
                let wrap = |middle: &str| {
                    format!(
                        r#"<{prefix}:{container} xmlns:{prefix}="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:m="http://schemas.openxmlformats.org/officeDocument/2006/math" xmlns:f="urn:metadata">{before}{middle}{after}</{prefix}:{container}>"#
                    )
                };
                let control = read_container(&wrap("")).unwrap();
                let math = r#"<m:oMath><m:r><m:t>E=mc</m:t></m:r><m:sSup><m:e><m:r><m:t>x</m:t></m:r></m:e><m:sup><m:r><m:t>2</m:t></m:r></m:sup></m:sSup></m:oMath><f:r><f:t>Metadata</f:t></f:r>"#;
                let tail = wrap("").replace(
                    &format!("</{prefix}:{container}>"),
                    &format!("{math}</{prefix}:{container}>"),
                );
                for xml in [wrap(math), tail] {
                    let actual = read_container(&xml).unwrap();
                    assert_eq!(actual, control, "container {container}, prefix {prefix}");
                }
            }
        }
    }

    #[test]
    fn read_run_keeps_page_breaks_with_local_namespace_declarations() {
        for prefix in ["text", "type"] {
            let xml = format!(
                r#"<w:r xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:t>Before</w:t><{prefix}:br xmlns:{prefix}="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:f="urn:metadata" f:type="column" {prefix}:clear="all" {prefix}:type="page"/><w:t>After</w:t></w:r>"#
            );
            let mut reader = EventReader::new(xml.as_bytes());
            let run = Run::read(&mut reader, &[]).unwrap();
            assert_eq!(
                run,
                Run::new()
                    .add_text("Before")
                    .add_break(BreakType::Page)
                    .add_text("After")
            );
        }
    }

    #[test]
    fn read_run_local_declarations_do_not_replace_property_values() {
        let xml = r#"<w:r xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:rPr><val:b xmlns:val="http://schemas.openxmlformats.org/wordprocessingml/2006/main" val:val="0"/></w:rPr><w:t>Regular report text</w:t></w:r>"#;
        let mut reader = EventReader::new(xml.as_bytes());
        assert_eq!(
            Run::read(&mut reader, &[]).unwrap(),
            Run::new().disable_bold().add_text("Regular report text")
        );
    }

    #[test]
    fn read_run_uses_wordprocessing_namespace_with_any_prefix() {
        for (opening, prefix) in [
            (
                "xmlns:ns0=\"http://schemas.openxmlformats.org/wordprocessingml/2006/main\"",
                "ns0:",
            ),
            (
                "xmlns=\"http://schemas.openxmlformats.org/wordprocessingml/2006/main\"",
                "",
            ),
            (
                "xmlns:text=\"http://purl.oclc.org/ooxml/wordprocessingml/main\"",
                "text:",
            ),
        ] {
            let xml = format!("<{prefix}r {opening}><{prefix}rPr><{prefix}b/></{prefix}rPr><{prefix}t>Quarterly report</{prefix}t><{prefix}tab/><{prefix}t>Approved</{prefix}t></{prefix}r>");
            let mut reader = EventReader::new(xml.as_bytes());
            let run = Run::read(&mut reader, &[]).unwrap();
            let expected = Run::new()
                .bold()
                .add_text("Quarterly report")
                .add_tab()
                .add_text("Approved");
            assert_eq!(run, expected, "prefix {prefix}");
        }
    }

    #[test]
    fn read_run_ignores_foreign_names_and_restores_scoped_bindings() {
        let xml = r#"<w:r xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:f="urn:foreign">
          <w:t>Before</w:t><f:r><f:t>Foreign</f:t></f:r>
          <w:t xmlns:w="urn:foreign">Shadowed</w:t>
          <w:t>After</w:t>
        </w:r>"#;
        let mut reader = EventReader::new(xml.as_bytes());
        let run = Run::read(&mut reader, &[]).unwrap();
        assert_eq!(run, Run::new().add_text("Before").add_text("After"));
    }

    #[test]
    fn read_run_rejects_an_unrecognized_root_without_looping_at_eof() {
        let xml = r#"<f:r xmlns:f="urn:metadata"><f:t>Metadata</f:t></f:r>"#;
        let mut reader = EventReader::new(xml.as_bytes());
        assert!(Run::read(&mut reader, &[]).is_err());
    }

    #[test]
    fn test_read_size_color() {
        let c = r#"<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:r><w:rPr><w:color w:val="C9211E"/><w:sz w:val="30"/><w:szCs w:val="30"/></w:rPr><w:t>H</w:t></w:r>
</w:document>"#;
        let mut parser = EventReader::new(c.as_bytes());
        let run = Run::read(&mut parser, &[]).unwrap();
        assert_eq!(
            run,
            Run {
                children: vec![RunChild::Text(Text::new("H"))],
                run_property: RunProperty {
                    sz: Some(Sz::new(30)),
                    sz_cs: Some(SzCs::new(30)),
                    color: Some(Color::new("C9211E")),
                    ..RunProperty::default()
                },
            }
        );
    }

    #[test]
    fn test_read_theme_color() {
        // The reader must pick attributes by name (not position) and round-trip
        // w:themeColor / w:themeShade alongside the w:val fallback.
        let c = r#"<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:r><w:rPr><w:color w:val="2E74B5" w:themeColor="accent1" w:themeShade="BF"/></w:rPr><w:t>H</w:t></w:r>
</w:document>"#;
        let mut parser = EventReader::new(c.as_bytes());
        let run = Run::read(&mut parser, &[]).unwrap();
        assert_eq!(
            run,
            Run {
                children: vec![RunChild::Text(Text::new("H"))],
                run_property: RunProperty {
                    color: Some(
                        Color::new("2E74B5")
                            .theme_color(crate::ThemeColor::Accent1)
                            .theme_shade("BF")
                    ),
                    ..RunProperty::default()
                },
            }
        );
    }

    #[test]
    fn test_read_tab() {
        let c = r#"<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:r><w:tab /></w:r>
</w:document>"#;
        let mut parser = EventReader::new(c.as_bytes());
        let run = Run::read(&mut parser, &[]).unwrap();
        assert_eq!(
            run,
            Run {
                children: vec![RunChild::Tab(Tab::new())],
                run_property: RunProperty::default(),
            }
        );
    }

    #[test]
    fn test_read_br() {
        let c = r#"<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:r><w:br w:type="page" /></w:r>
</w:document>"#;
        let mut parser = EventReader::new(c.as_bytes());
        let run = Run::read(&mut parser, &[]).unwrap();
        assert_eq!(
            run,
            Run {
                children: vec![RunChild::Break(Break::new(BreakType::Page))],
                run_property: RunProperty::default(),
            }
        );
    }

    #[test]
    fn test_read_empty_br() {
        let c = r#"<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:r><w:br /></w:r>
</w:document>"#;
        let mut parser = EventReader::new(c.as_bytes());
        let run = Run::read(&mut parser, &[]).unwrap();
        assert_eq!(
            run,
            Run {
                children: vec![RunChild::Break(Break::new(BreakType::TextWrapping))],
                run_property: RunProperty::default(),
            }
        );
    }

    #[test]
    fn test_read_cr() {
        let c = r#"<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:r><w:cr /></w:r>
</w:document>"#;
        let mut parser = EventReader::new(c.as_bytes());
        let run = Run::read(&mut parser, &[]).unwrap();
        assert_eq!(
            run,
            Run {
                children: vec![RunChild::CarriageReturn(CarriageReturn::new())],
                run_property: RunProperty::default(),
            }
        );
    }

    #[test]
    fn test_read_italic_false() {
        let c = r#"<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:r><w:rPr>
    <w:b w:val="true"/>
    <w:i w:val="false"/>
  </w:rPr></w:r>
</w:document>"#;
        let mut parser = EventReader::new(c.as_bytes());
        let run = Run::read(&mut parser, &[]).unwrap();
        assert_eq!(
            run,
            Run {
                children: vec![],
                run_property: RunProperty {
                    bold: Some(Bold::new()),
                    bold_cs: Some(BoldCs::new()),
                    italic: Some(Italic::new().disable()),
                    italic_cs: Some(ItalicCs::new().disable()),
                    ..RunProperty::default()
                },
            }
        );
    }

    #[test]
    fn test_read_italic_0() {
        let c = r#"<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:r><w:rPr>
    <w:b w:val="1"/>
    <w:i w:val="0"/>
  </w:rPr></w:r>
</w:document>"#;
        let mut parser = EventReader::new(c.as_bytes());
        let run = Run::read(&mut parser, &[]).unwrap();
        assert_eq!(
            run,
            Run {
                children: vec![],
                run_property: RunProperty {
                    bold: Some(Bold::new()),
                    bold_cs: Some(BoldCs::new()),
                    italic: Some(Italic::new().disable()),
                    italic_cs: Some(ItalicCs::new().disable()),
                    ..RunProperty::default()
                },
            }
        );
    }

    #[test]
    fn test_read_on_off_values() {
        let c = r#"<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:r><w:rPr>
    <w:b w:val="off"/>
    <w:i w:val="on"/>
  </w:rPr></w:r>
</w:document>"#;
        let mut parser = EventReader::new(c.as_bytes());
        let run = Run::read(&mut parser, &[]).unwrap();
        assert_eq!(
            run,
            Run {
                children: vec![],
                run_property: RunProperty {
                    bold: Some(Bold::new().disable()),
                    bold_cs: Some(BoldCs::new().disable()),
                    italic: Some(Italic::new()),
                    italic_cs: Some(ItalicCs::new()),
                    ..RunProperty::default()
                },
            }
        );
    }

    #[test]
    fn test_read_fit_text() {
        let c = r#"<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:r><w:rPr>
    <w:fitText w:val="840" w:id="1266434317"/>
  </w:rPr></w:r>
</w:document>"#;
        let mut parser = EventReader::new(c.as_bytes());
        let run = Run::read(&mut parser, &[]).unwrap();
        assert_eq!(
            run,
            Run {
                children: vec![],
                run_property: RunProperty {
                    fit_text: Some(FitText::new(840).id(1266434317)),
                    ..RunProperty::default()
                },
            }
        );
    }
}
