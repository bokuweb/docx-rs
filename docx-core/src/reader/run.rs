#![allow(clippy::single_match)]

use std::io::Read;
use std::str::FromStr;

use xml::attribute::OwnedAttribute;
use xml::reader::{EventReader, XmlEvent};

use super::Run;

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

impl ElementReader for Run {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        _attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut run = Run::new();
        let mut text_state = TextState::Idle;
        loop {
            let e = r.next();
            match e {
                Ok(XmlEvent::StartElement {
                    attributes, name, ..
                }) => {
                    match name.prefix.as_deref() {
                        Some("w") => {
                            let e = XMLElement::from_str(&name.local_name).unwrap();

                            ignore::ignore_element(e.clone(), XMLElement::RunPropertyChange, r);

                            match e {
                                XMLElement::Tab => {
                                    run = run.add_tab();
                                }
                                XMLElement::RunProperty => {
                                    let p = RunProperty::read(r, &attributes)?;
                                    run = run.set_property(p);
                                }
                                XMLElement::Text => text_state = TextState::Text,
                                XMLElement::DeleteText => text_state = TextState::Delete,
                                XMLElement::Break => {
                                    if let Some(a) = &attributes.get(0) {
                                        run = run.add_break(BreakType::from_str(&a.value)?)
                                    } else {
                                        run = run.add_break(BreakType::TextWrapping)
                                    }
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
                                XMLElement::InstrText => {
                                    if let Ok(i) = InstrText::read(r, &attributes) {
                                        run.children.push(RunChild::InstrText(Box::new(i)));
                                    }
                                }
                                _ => {}
                            }
                        }
                        Some("mc") => {
                            let e = McXMLElement::from_str(&name.local_name).unwrap();
                            match e {
                                McXMLElement::Fallback => {
                                    let _ = McFallback::read(r, &attributes)?;
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    };
                }
                Ok(XmlEvent::Characters(c)) => match text_state {
                    TextState::Delete => {
                        run = run.add_delete_text(c);
                    }
                    TextState::Text => {
                        run = run.add_text(c);
                    }
                    _ => {}
                },
                Ok(XmlEvent::Whitespace(c)) => match text_state {
                    TextState::Delete => {
                        run = run.add_delete_text(c);
                    }
                    TextState::Text => {
                        run = run.add_text(c);
                    }
                    _ => {}
                },
                Ok(XmlEvent::EndElement { name, .. }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    match e {
                        XMLElement::Run => {
                            return Ok(run);
                        }
                        XMLElement::DeleteText | XMLElement::Text => text_state = TextState::Idle,
                        _ => {}
                    }
                }
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;

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
}
