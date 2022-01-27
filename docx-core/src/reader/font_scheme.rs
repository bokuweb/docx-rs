#![allow(clippy::single_match)]

use std::io::Read;
use std::str::FromStr;

use xml::attribute::OwnedAttribute;
use xml::reader::{EventReader, XmlEvent};

use crate::reader::*;

impl ElementReader for FontScheme {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        _attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut fs = FontScheme::new();
        loop {
            let e = r.next();
            match e {
                Ok(XmlEvent::StartElement {
                    attributes, name, ..
                }) => {
                    match name.prefix.as_deref() {
                        Some("a") => {
                            let e = AXMLElement::from_str(&name.local_name).unwrap();
                            match e {
                                AXMLElement::MajorFont => {
                                    if let Ok(f) = FontGroup::read(r, &attributes) {
                                        fs.major_font = f;
                                    }
                                }
                                AXMLElement::MinorFont => {
                                    if let Ok(f) = FontGroup::read(r, &attributes) {
                                        fs.minor_font = f;
                                    }
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    };
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    let e = AXMLElement::from_str(&name.local_name).unwrap();
                    match e {
                        AXMLElement::FontScheme => {
                            return Ok(fs);
                        }
                        _ => {}
                    }
                }
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
    }
}
