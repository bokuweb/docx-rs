use std::io::Read;
use std::str::FromStr;

use xml::attribute::OwnedAttribute;
use xml::reader::{EventReader, XmlEvent};

use super::*;

impl ElementReader for Level {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let level = read_indent_level(attrs)?;
        let mut style_id = None;
        let mut start = Start::default();
        let mut num_fmt = NumberFormat::new("decimal");
        let mut level_text = LevelText::new("");
        let mut jc = LevelJc::new("left");

        let mut indent_start = None;
        let mut special_indent = None;
        let mut indent_end = None;
        let mut start_chars = None;
        let mut has_indent = false;

        loop {
            let e = r.next();
            match e {
                Ok(XmlEvent::StartElement {
                    attributes, name, ..
                }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    match e {
                        XMLElement::ParagraphStyle => {
                            let id = attributes[0].value.clone();
                            style_id = Some(id);
                        }
                        XMLElement::Start => {
                            start = Start::new(usize::from_str(&attributes[0].value)?);
                        }
                        XMLElement::NumberFormat => {
                            num_fmt = NumberFormat::new(attributes[0].value.clone());
                        }
                        XMLElement::LevelText => {
                            level_text = LevelText::new(attributes[0].value.clone());
                        }
                        XMLElement::LevelJustification => {
                            jc = LevelJc::new(attributes[0].value.clone());
                        }
                        XMLElement::Indent => {
                            let i = read_indent(&attributes)?;
                            indent_start = i.0;
                            indent_end = i.1;
                            special_indent = i.2;
                            start_chars = i.3;
                            has_indent = true;
                        }
                        _ => {}
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    if let XMLElement::Level = e {
                        let mut l = Level::new(level, start, num_fmt, level_text, jc);
                        if let Some(style_id) = style_id {
                            l = l.paragraph_style(style_id);
                        }
                        if has_indent {
                            l = l.indent(indent_start, special_indent, indent_end, start_chars);
                        }
                        return Ok(l);
                    }
                }
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
    }
}
