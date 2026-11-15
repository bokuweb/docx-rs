use serde::ser::{Serialize, SerializeStruct, Serializer};
use serde::Deserialize;
use std::io::Write;

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct NoBreakHyphen {}

impl Default for NoBreakHyphen {
    fn default() -> Self {
        Self::new()
    }
}

impl NoBreakHyphen {
    pub fn new() -> NoBreakHyphen {
        NoBreakHyphen {}
    }
}

impl BuildXML for NoBreakHyphen {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream).no_break_hyphen()?.into_inner()
    }
}

impl Serialize for NoBreakHyphen {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let t = serializer.serialize_struct("NoBreakHyphen", 0)?;
        t.end()
    }
}
