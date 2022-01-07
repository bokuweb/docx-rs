use std::io::Read;
use std::str::FromStr;

use xml::attribute::OwnedAttribute;
use xml::reader::{EventReader, XmlEvent};

use super::*;

impl ElementReader for ParagraphPropertyChange {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut pc = ParagraphPropertyChange::new();
        for attr in attrs {
            let local_name = &attr.name.local_name;
            if local_name == "author" {
                pc = pc.author(&attr.value);
            } else if local_name == "date" {
                pc = pc.date(&attr.value);
            }
        }
        loop {
            let e = r.next();
            match e {
                Ok(XmlEvent::StartElement { name, .. }) => {
                    let e = XMLElement::from_str(&name.local_name)
                        .expect("should convert to XMLElement");
                    if let XMLElement::ParagraphProperty = e {
                        if let Ok(p) = ParagraphProperty::read(r, attrs) {
                            pc = pc.property(p);
                        }
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    if e == XMLElement::ParagraphPropertyChange {
                        return Ok(pc);
                    }
                }
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
    }
}
