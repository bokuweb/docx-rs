#![allow(clippy::single_match)]

use std::io::Read;
use std::str::FromStr;

use xml::attribute::OwnedAttribute;
use xml::reader::{EventReader, XmlEvent};

use super::*;

impl ElementReader for WpsShape {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        _attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut shape = WpsShape::new();
        loop {
            let e = r.next();
            match e {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    let e = WpsXMLElement::from_str(&name.local_name)
                        .expect("should convert to XMLElement");
                    match e {
                        WpsXMLElement::Txbx => {
                            let text_box = WpsTextBox::read(r, &attributes)?;
                            shape = shape.add_text_box(text_box);
                        }
                        _ => {}
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    let e = WpsXMLElement::from_str(&name.local_name).unwrap();
                    if e == WpsXMLElement::Wsp {
                        return Ok(shape);
                    }
                }
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
    }
}
