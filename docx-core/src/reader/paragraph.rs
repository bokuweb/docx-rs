use std::io::Read;
use std::str::FromStr;

use xml::attribute::OwnedAttribute;
use xml::reader::{EventReader, XmlEvent};

use super::Paragraph;

use crate::reader::*;
use crate::xml_builder::*;

pub struct ParagraphReader {}

#[derive(Debug, Clone)]
pub enum ParagraphReaderState {
    Ready,
    Run,
}

impl ElementReader for Paragraph {
    fn read<R: Read>(r: &mut EventReader<R>) -> Result<Self, ReaderError> {
        let mut p = Paragraph::new();
        loop {
            let e = r.next();
            match e {
                Ok(XmlEvent::StartElement {
                    attributes, name, ..
                }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    match e {
                        XMLElement::Run => {
                            let run = Run::read(r)?;
                            p = p.add_run(run);
                            continue;
                        }
                        _ => {}
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    match e {
                        XMLElement::Paragraph => return Ok(p),
                        _ => {}
                    }
                }
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
    }
}
