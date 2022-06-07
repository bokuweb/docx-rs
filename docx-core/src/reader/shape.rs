#![allow(clippy::single_match)]

use std::io::Read;
use std::str::FromStr;

use xml::attribute::OwnedAttribute;
use xml::reader::{EventReader, XmlEvent};

use super::*;

impl ElementReader for Shape {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut shape = Shape::new();
        if let Some(style) = read(attrs, "style") {
            shape = shape.style(style);
        }

        loop {
            let e = r.next();
            match e {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    if let Ok(VXMLElement::ImageData) = VXMLElement::from_str(&name.local_name) {
                        if let Some(id) = read(&attributes, "id") {
                            shape = shape.image_data(id);
                        }
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    let e = VXMLElement::from_str(&name.local_name).unwrap();
                    if e == VXMLElement::Shape {
                        return Ok(shape);
                    }
                }
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
    }
}
