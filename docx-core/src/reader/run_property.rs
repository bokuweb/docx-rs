use std::io::Read;
use std::str::FromStr;

use xml::attribute::OwnedAttribute;
use xml::reader::{EventReader, XmlEvent};

use crate::VertAlignType;

use super::*;

impl ElementReader for RunProperty {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        _attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut rp = RunProperty::new();
        loop {
            let e = r.next();
            match e {
                Ok(XmlEvent::StartElement {
                    attributes, name, ..
                }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    match e {
                        XMLElement::Bold => {
                            if !read_bool(&attributes) {
                                rp = rp.disable_bold();
                                continue;
                            }
                            rp = rp.bold();
                        }
                        XMLElement::Highlight => rp = rp.highlight(attributes[0].value.clone()),
                        XMLElement::VertAlign => {
                            if let Ok(v) = VertAlignType::from_str(&attributes[0].value) {
                                rp = rp.vert_align(v)
                            }
                        }
                        XMLElement::Color => rp = rp.color(attributes[0].value.clone()),
                        XMLElement::Size => rp = rp.size(usize::from_str(&attributes[0].value)?),
                        XMLElement::Spacing => {
                            if let Some(v) = read_val(&attributes) {
                                let v = value_to_dax(&v)?;
                                rp = rp.spacing(v)
                            }
                        }
                        // TODO: Implement later
                        XMLElement::RunFonts => {}
                        XMLElement::Underline => rp = rp.underline(&attributes[0].value.clone()),
                        XMLElement::Italic => {
                            if !read_bool(&attributes) {
                                rp = rp.disable_italic();
                                continue;
                            }
                            rp = rp.italic();
                        }
                        XMLElement::Vanish => rp = rp.vanish(),
                        XMLElement::TextBorder => {
                            if let Ok(attr) = read_border(&attributes) {
                                let mut border = TextBorder::new()
                                    .border_type(attr.border_type)
                                    .color(attr.color);
                                if let Some(size) = attr.size {
                                    border = border.size(size as usize);
                                };
                                rp = rp.text_border(border);
                                continue;
                            }
                        }
                        XMLElement::Insert => {
                            if let Ok(ins) = Insert::read(r, &attributes) {
                                rp = rp.insert(ins);
                            }
                        }
                        XMLElement::Delete => {
                            if let Ok(del) = Delete::read(r, &attributes) {
                                rp = rp.delete(del);
                            }
                        }
                        _ => {}
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    if e == XMLElement::RunProperty {
                        return Ok(rp);
                    }
                }
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
    }
}
