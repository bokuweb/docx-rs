use std::io::Read;
use std::str::FromStr;

use xml::attribute::OwnedAttribute;
use xml::reader::{EventReader, XmlEvent};

use super::Run;

use crate::reader::*;
use crate::types::BreakType;

#[derive(PartialEq, Debug)]
enum TextState {
    Idle,
    Text,
    Delete,
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
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    match e {
                        XMLElement::Tab => {
                            run = run.add_tab();
                        }
                        XMLElement::Bold => {
                            if !read_bool(&attributes) {
                                continue;
                            }
                            run = run.bold();
                        }
                        XMLElement::Highlight => run = run.highlight(attributes[0].value.clone()),
                        XMLElement::Color => run = run.color(attributes[0].value.clone()),
                        XMLElement::Size => run = run.size(usize::from_str(&attributes[0].value)?),
                        XMLElement::Underline => run = run.underline(&attributes[0].value.clone()),
                        XMLElement::Italic => {
                            if !read_bool(&attributes) {
                                continue;
                            }
                            run = run.italic();
                        }
                        XMLElement::Vanish => run = run.vanish(),
                        XMLElement::Text => text_state = TextState::Text,
                        XMLElement::DeleteText => text_state = TextState::Delete,
                        XMLElement::Break => {
                            if let Some(a) = &attributes.get(0) {
                                run = run.add_break(BreakType::from_str(&a.value)?)
                            } else {
                                run = run.add_break(BreakType::TextWrapping)
                            }
                        }
                        _ => {}
                    }
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
                    highlight: None,
                    underline: None,
                    bold: None,
                    bold_cs: None,
                    italic: None,
                    italic_cs: None,
                    vanish: None,
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
                run_property: RunProperty {
                    sz: None,
                    sz_cs: None,
                    color: None,
                    highlight: None,
                    underline: None,
                    bold: None,
                    bold_cs: None,
                    italic: None,
                    italic_cs: None,
                    vanish: None,
                },
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
                run_property: RunProperty {
                    sz: None,
                    sz_cs: None,
                    color: None,
                    highlight: None,
                    underline: None,
                    bold: None,
                    bold_cs: None,
                    italic: None,
                    italic_cs: None,
                    vanish: None,
                },
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
                run_property: RunProperty {
                    sz: None,
                    sz_cs: None,
                    color: None,
                    highlight: None,
                    underline: None,
                    bold: None,
                    bold_cs: None,
                    italic: None,
                    italic_cs: None,
                    vanish: None,
                },
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
                    sz: None,
                    sz_cs: None,
                    color: None,
                    highlight: None,
                    underline: None,
                    bold: Some(Bold::new()),
                    bold_cs: Some(BoldCs::new()),
                    italic: None,
                    italic_cs: None,
                    vanish: None,
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
                    sz: None,
                    sz_cs: None,
                    color: None,
                    highlight: None,
                    underline: None,
                    bold: Some(Bold::new()),
                    bold_cs: Some(BoldCs::new()),
                    italic: None,
                    italic_cs: None,
                    vanish: None,
                },
            }
        );
    }
}
