use serde::{Deserialize, Serialize, Serializer};
use std::io::Write;

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, Deserialize, PartialEq, Default)]
pub struct SpecVanish {}

impl SpecVanish {
    pub fn new() -> SpecVanish {
        SpecVanish {}
    }
}

impl BuildXML for SpecVanish {
    fn build_to<W: Write>(
        &self,
        stream: xml::writer::EventWriter<W>,
    ) -> xml::writer::Result<xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream).spec_vanish()?.into_inner()
    }
}

impl Serialize for SpecVanish {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bool(true)
    }
}
