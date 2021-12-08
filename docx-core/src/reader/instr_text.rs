#![allow(clippy::single_match)]

use std::io::Read;
use std::str::FromStr;

use xml::attribute::OwnedAttribute;
use xml::reader::{EventReader, XmlEvent};

use crate::reader::*;

impl ElementReader for InstrText {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        _attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut instr = "".to_owned();
        loop {
            let e = r.next();
            match e {
                Ok(XmlEvent::Characters(c)) => {
                    instr = c;
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    match e {
                        XMLElement::InstrText => {
                            let instr = instr.trim();
                            if instr.is_empty() {
                                return Err(ReaderError::XMLReadError);
                            } else {
                                if instr.starts_with("TOC") {
                                    for i in instr.split(' ') {
                                        dbg!(i);
                                    }
                                    return Ok(InstrText::TOC(InstrToC::new()));
                                }
                                return Ok(InstrText::Unsupported(instr.to_string()));
                            }
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
