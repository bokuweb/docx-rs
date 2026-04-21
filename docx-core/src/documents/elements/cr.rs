use serde::ser::{Serialize, SerializeStruct, Serializer};
use serde::Deserialize;
use std::io::Write;

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct CarriageReturn {}

impl CarriageReturn {
    pub fn new() -> CarriageReturn {
        CarriageReturn {}
    }
}

impl BuildXML for CarriageReturn {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream).cr()?.into_inner()
    }
}

impl Serialize for CarriageReturn {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let t = serializer.serialize_struct("CarriageReturn", 0)?;
        t.end()
    }
}
