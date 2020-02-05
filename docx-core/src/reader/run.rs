use std::io::Read;
use std::str::FromStr;

use xml::attribute::OwnedAttribute;
use xml::reader::{EventReader, XmlEvent};

use super::Run;

use crate::reader::*;

impl ElementReader for Run {
    fn read<R: Read>(r: &mut EventReader<R>) -> Result<Self, ReaderError> {
        let mut run = Run::new();
        loop {
            let e = r.next();
            match e {
                Ok(XmlEvent::StartElement {
                    attributes, name, ..
                }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    dbg!(&e);
                    match e {
                        XMLElement::RunProperty => {}
                        _ => {}
                    }
                }
                Ok(XmlEvent::Characters(c)) => {
                    run = run.add_text(c);
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    match e {
                        XMLElement::Run => {
                            run = run.vanish();
                            dbg!(serde_json::to_string(&run).unwrap());
                            return Ok(run);
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
