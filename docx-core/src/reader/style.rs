use std::io::Read;
use std::str::FromStr;

use xml::attribute::OwnedAttribute;
use xml::reader::{EventReader, XmlEvent};

use super::*;

use crate::types::*;

impl ElementReader for Style {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut id = "".to_owned();
        let mut style_type = StyleType::Paragraph;
        for a in attrs {
            let local_name = &a.name.local_name;
            if local_name == "styleId" {
                id = a.value.clone();
            } else if local_name == "type" {
                style_type = StyleType::from_str(&a.value)?;
            }
        }
        let mut style = Style::new(id, style_type);
        loop {
            let e = r.next();
            match e {
                Ok(XmlEvent::StartElement {
                    attributes, name, ..
                }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    match e {
                        XMLElement::Name => {
                            style = style.name(&attributes[0].value.clone());
                            continue;
                        }
                        // pPr
                        XMLElement::Indent => {
                            let (start, end, special, start_chars) = read_indent(&attributes)?;
                            style = style.indent(start, special, end, start_chars);
                            continue;
                        }
                        XMLElement::Justification => {
                            style = style.align(AlignmentType::from_str(&attributes[0].value)?);
                            continue;
                        }
                        // rPr
                        XMLElement::Bold => {
                            if !read_bool(&attributes) {
                                continue;
                            }
                            style = style.bold();
                        }
                        XMLElement::Highlight => {
                            style = style.highlight(attributes[0].value.clone())
                        }
                        XMLElement::Color => style = style.color(attributes[0].value.clone()),
                        XMLElement::Size => {
                            style = style.size(usize::from_str(&attributes[0].value)?)
                        }
                        XMLElement::Underline => {
                            style = style.underline(&attributes[0].value.clone())
                        }
                        XMLElement::Italic => {
                            if !read_bool(&attributes) {
                                continue;
                            }
                            style = style.italic();
                        }
                        XMLElement::Vanish => style = style.vanish(),
                        _ => {}
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    if let XMLElement::Style = e {
                        return Ok(style);
                    }
                }
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
    }
}
