use serde::ser::{Serialize, SerializeStruct, Serializer};
use serde::Deserialize;
use std::io::Write;

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct SoftHyphen {}

impl Default for SoftHyphen {
    fn default() -> Self {
        Self::new()
    }
}

impl SoftHyphen {
    pub fn new() -> SoftHyphen {
        SoftHyphen {}
    }
}

impl BuildXML for SoftHyphen {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream).soft_hyphen()?.into_inner()
    }
}

impl Serialize for SoftHyphen {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let t = serializer.serialize_struct("SoftHyphen", 0)?;
        t.end()
    }
}
