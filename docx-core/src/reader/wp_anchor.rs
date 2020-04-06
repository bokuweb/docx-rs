use std::io::Read;
use std::str::FromStr;

use xml::attribute::OwnedAttribute;
use xml::reader::{EventReader, XmlEvent};

use super::*;

impl ElementReader for WpAnchor {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        _attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut anchor = WpAnchor::new();
        loop {
            let e = r.next();
            match e {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    let e = AXMLElement::from_str(&name.local_name)
                        .expect("should convert to XMLElement");
                    if let AXMLElement::Graphic = e {
                        let g = AGraphic::read(r, &attributes)?;
                        anchor = anchor.add_graphic(g);
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    let e = WpXMLElement::from_str(&name.local_name).unwrap();
                    if e == WpXMLElement::Anchor {
                        return Ok(anchor);
                    }
                }
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
    }
}
