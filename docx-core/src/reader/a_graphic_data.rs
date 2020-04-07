#![allow(clippy::single_match)]

use std::io::Read;
use std::str::FromStr;

use xml::attribute::OwnedAttribute;
use xml::reader::{EventReader, XmlEvent};

use super::*;

impl ElementReader for AGraphicData {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut t = GraphicDataType::Unsupported;
        for a in attrs {
            if a.name.local_name == "uri" {
                t = GraphicDataType::from_str(&a.value).unwrap();
            }
        }
        let mut graphic_data = AGraphicData::new(t);
        loop {
            let e = r.next();
            match e {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    let e = WpsXMLElement::from_str(&name.local_name)
                        .expect("should convert to XMLElement");
                    match e {
                        WpsXMLElement::Wsp => {
                            let shape = WpsShape::read(r, &attributes)?;
                            graphic_data = graphic_data.add_shape(shape);
                        }
                        _ => {}
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    let e = AXMLElement::from_str(&name.local_name).unwrap();
                    if e == AXMLElement::GraphicData {
                        return Ok(graphic_data);
                    }
                }
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
    }
}
