use std::io::Read;
use std::str::FromStr;

use xml::attribute::OwnedAttribute;
use xml::reader::{EventReader, XmlEvent};

use super::*;

impl ElementReader for AGraphic {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut graphic = AGraphic::new();
        loop {
            let e = r.next();
            match e {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    let e = AXMLElement::from_str(&name.local_name)
                        .expect("should convert to XMLElement");
                    if let AXMLElement::GraphicData = e {
                        let data = AGraphicData::read(r, &attributes)?;
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    let e = AXMLElement::from_str(&name.local_name).unwrap();
                    if e == AXMLElement::Graphic {
                        return Ok(graphic);
                    }
                }
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
    }
}
