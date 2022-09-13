use std::io::Read;
use std::str::FromStr;

use xml::attribute::OwnedAttribute;
use xml::reader::{EventReader, XmlEvent};

use super::*;

impl ElementReader for LevelOverride {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        if let Ok(level) = usize::from_str(&attrs[0].value) {
            let mut o = LevelOverride::new(level);
            loop {
                let e = r.next();
                match e {
                    Ok(XmlEvent::StartElement {
                        attributes, name, ..
                    }) => {
                        let e = XMLElement::from_str(&name.local_name).unwrap();
                        match e {
                            XMLElement::StartOverride => {
                                if let Ok(val) = usize::from_str(&attributes[0].value) {
                                    o = o.start(val);
                                }
                                continue;
                            }
                            XMLElement::Level => {
                                if let Ok(lvl) = Level::read(r, &attributes) {
                                    o = o.level(lvl);
                                }
                                continue;
                            }
                            _ => {}
                        }
                    }
                    Ok(XmlEvent::EndElement { name, .. }) => {
                        let e = XMLElement::from_str(&name.local_name).unwrap();
                        if e == XMLElement::LvlOverride {
                            return Ok(o);
                        }
                    }
                    Err(_) => return Err(ReaderError::XMLReadError),
                    _ => {}
                }
            }
        } else {
            Err(ReaderError::XMLReadError)
        }
    }
}
