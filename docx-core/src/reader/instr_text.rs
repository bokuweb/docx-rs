#![allow(clippy::single_match)]

use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;

use std::io::Read;

use xml::attribute::OwnedAttribute;
use xml::reader::{EventReader, XmlEvent};

use crate::reader::*;

// For reader only
#[derive(PartialEq, Debug)]
pub struct InstrText(String);

impl Serialize for InstrText {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut t = serializer.serialize_struct("InstrText", 2)?;
        t.serialize_field("type", "instrText")?;
        t.serialize_field("data", &self.0)?;
        t.end()
    }
}

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
                Ok(XmlEvent::EndElement { name, .. }) => return Ok(InstrText(instr)),
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
    }
}
