use std::io::Read;
use std::str::FromStr;

use xml::attribute::OwnedAttribute;
use xml::reader::{EventReader, XmlEvent};

use super::*;

impl ElementReader for Insert {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut run = Run::new();
        loop {
            let e = r.next();
            match e {
                Ok(XmlEvent::StartElement { name, .. }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    if let XMLElement::Run = e {
                        run = Run::read(r, attrs)?;
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    if e == XMLElement::Insert {
                        let mut ins = Insert::new(run);
                        for attr in attrs {
                            let local_name = &attr.name.local_name;
                            if local_name == "author" {
                                ins = ins.author(&attr.value);
                            } else if local_name == "date" {
                                ins = ins.date(&attr.value);
                            }
                        }
                        return Ok(ins);
                    }
                }
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
    }
}
