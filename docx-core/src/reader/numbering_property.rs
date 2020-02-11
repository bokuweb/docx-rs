use std::io::Read;
use std::str::FromStr;

use xml::attribute::OwnedAttribute;
use xml::reader::{EventReader, XmlEvent};

use super::*;

impl ElementReader for NumberingProperty {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        _attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut id: Option<usize> = None;
        let mut level: Option<usize> = None;
        loop {
            let e = r.next();
            match e {
                Ok(XmlEvent::StartElement {
                    attributes, name, ..
                }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    match e {
                        XMLElement::IndentLevel => {
                            level = Some(usize::from_str(&attributes[0].value)?);
                            continue;
                        }
                        XMLElement::NumberingId => {
                            id = Some(usize::from_str(&attributes[0].value)?);
                            continue;
                        }
                        _ => {}
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    if e == XMLElement::NumberingProperty {
                        if id.is_none() || level.is_none() {
                            return Err(ReaderError::XMLReadError);
                        }
                        let np = NumberingProperty::new(
                            NumberingId::new(id.unwrap()),
                            IndentLevel::new(level.unwrap()),
                        );
                        return Ok(np);
                    }
                }
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
    }
}
