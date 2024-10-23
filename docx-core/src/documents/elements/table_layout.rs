use serde::{Serialize, Serializer};
use std::io::Write;

use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;

#[derive(Debug, Clone, PartialEq)]
pub struct TableLayout {
    layout_type: TableLayoutType,
}

impl TableLayout {
    pub fn new(t: TableLayoutType) -> TableLayout {
        TableLayout { layout_type: t }
    }
}

impl BuildXML for TableLayout {
    fn build_to<W: Write>(
        &self,
        stream: xml::writer::EventWriter<W>,
    ) -> xml::writer::Result<xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .table_layout(&self.layout_type.to_string())?
            .into_inner()
    }
}

impl Serialize for TableLayout {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.layout_type.to_string())
    }
}
