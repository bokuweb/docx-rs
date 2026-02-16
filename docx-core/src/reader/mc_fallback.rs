#![allow(clippy::single_match)]

use std::io::Read;
use std::str::FromStr;

use super::McFallback;

use crate::reader::*;

impl ElementReader for McFallback {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        _attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        loop {
            let fallback = McFallback::new();
            let e = r.next();
            match e {
                Ok(XmlEvent::EndElement { name, .. }) => {
                    let e = McXMLElement::from_str(&name.local_name).unwrap();
                    match e {
                        McXMLElement::Fallback => {
                            return Ok(fallback);
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
