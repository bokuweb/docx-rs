use std::io::Read;
use std::str::FromStr;

use xml::attribute::OwnedAttribute;
use xml::reader::{EventReader, XmlEvent};

use super::*;

impl ElementReader for Delete {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut run: Option<Run> = None;
        loop {
            let e = r.next();
            match e {
                Ok(XmlEvent::StartElement { name, .. }) => {
                    let e = XMLElement::from_str(&name.local_name)
                        .expect("should convert to XMLElement");
                    if let XMLElement::Run = e {
                        run = Some(Run::read(r, attrs)?);
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    if e == XMLElement::Delete {
                        if let Some(run) = run {
                            let mut del = Delete::new(run);
                            for attr in attrs {
                                let local_name = &attr.name.local_name;
                                if local_name == "author" {
                                    del = del.author(&attr.value);
                                } else if local_name == "date" {
                                    del = del.date(&attr.value);
                                }
                            }
                            return Ok(del);
                        } else {
                            return Err(ReaderError::XMLReadError);
                        }
                    }
                }
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
    }
}
