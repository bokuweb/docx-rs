use serde::ser::{Serialize, SerializeStruct, Serializer};
use serde::Deserialize;
use std::io::Write;

use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Break {
    break_type: BreakType,
}

impl Break {
    pub fn new(t: BreakType) -> Break {
        Break { break_type: t }
    }
}

impl BuildXML for Break {
    fn build_to<W: Write>(
        &self,
        stream: xml::writer::EventWriter<W>,
    ) -> xml::writer::Result<xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .br(&self.break_type.to_string())?
            .into_inner()
    }
}

impl Serialize for Break {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut t = serializer.serialize_struct("Break", 1)?;
        t.serialize_field("breakType", &format!("{}", &self.break_type))?;
        t.end()
    }
}
