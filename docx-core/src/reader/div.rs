use std::io::Read;
use std::str::FromStr;

use xml::attribute::OwnedAttribute;
use xml::reader::{EventReader, XmlEvent};

use super::*;

impl ElementReader for Div {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let id = read_id(attrs).unwrap_or_default();
        let mut div = Div::new(id);
        loop {
            let e = r.next();
            match e {
                Ok(XmlEvent::StartElement {
                    attributes, name, ..
                }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    match e {
                        XMLElement::MarginLeft => {
                            if let Some(val) = read_val(&attributes) {
                                if let Ok(val) = f32::from_str(&val) {
                                    div = div.margin_left(val as usize);
                                }
                            }
                        }
                        XMLElement::MarginRight => {
                            if let Some(val) = read_val(&attributes) {
                                if let Ok(val) = f32::from_str(&val) {
                                    div = div.margin_right(val as usize);
                                }
                            }
                        }
                        XMLElement::MarginTop => {
                            if let Some(val) = read_val(&attributes) {
                                if let Ok(val) = f32::from_str(&val) {
                                    div = div.margin_top(val as usize);
                                }
                            }
                        }
                        XMLElement::MarginBottom => {
                            if let Some(val) = read_val(&attributes) {
                                if let Ok(val) = f32::from_str(&val) {
                                    div = div.margin_bottom(val as usize);
                                }
                            }
                        }
                        XMLElement::DivsChild => loop {
                            let e = r.next();
                            match e {
                                Ok(XmlEvent::StartElement {
                                    attributes, name, ..
                                }) => {
                                    let e = XMLElement::from_str(&name.local_name).unwrap();
                                    if let XMLElement::Div = e {
                                        if let Ok(c) = Div::read(r, &attributes) {
                                            div = div.add_child(c)
                                        }
                                    }
                                }
                                Ok(XmlEvent::EndElement { name, .. }) => {
                                    let e = XMLElement::from_str(&name.local_name).unwrap();
                                    if let XMLElement::DivsChild = e {
                                        break;
                                    }
                                }
                                Err(_) => return Err(ReaderError::XMLReadError),
                                _ => {}
                            }
                        },
                        _ => {}
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    if let XMLElement::Div = e {
                        return Ok(div);
                    }
                }
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
    }
}
