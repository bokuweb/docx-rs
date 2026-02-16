use serde::{Serialize, Serializer};
use std::io::Write;

use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;

#[derive(Debug, Clone, PartialEq)]
pub struct VertAlign {
    val: VertAlignType,
}

impl VertAlign {
    pub fn new(val: VertAlignType) -> VertAlign {
        Self { val }
    }
}

impl Serialize for VertAlign {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{}", &self.val))
    }
}

impl BuildXML for VertAlign {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .vert_align(&self.val.to_string())?
            .into_inner()
    }
}
