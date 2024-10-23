use serde::{Serialize, Serializer};
use std::io::Write;

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, PartialEq)]
pub struct TableStyle {
    val: String,
}

impl TableStyle {
    pub fn new(val: impl Into<String>) -> TableStyle {
        TableStyle { val: val.into() }
    }
}

impl BuildXML for TableStyle {
    fn build_to<W: Write>(
        &self,
        stream: xml::writer::EventWriter<W>,
    ) -> xml::writer::Result<xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .table_style(&self.val)?
            .into_inner()
    }
}

impl Serialize for TableStyle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.val)
    }
}
