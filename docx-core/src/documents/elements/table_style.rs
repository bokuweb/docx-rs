use serde::{Serialize, Serializer};

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
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.table_style(&self.val).build()
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
