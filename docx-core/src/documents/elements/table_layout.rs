use serde::{Serialize, Serializer};

use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;

#[derive(Debug, Clone, PartialEq)]
pub struct TableLayout {
    layout_type: TableLayoutType,
}

impl TableLayout {
    pub fn new(t: TableLayoutType) -> TableLayout {
        TableLayout {
            layout_type: t,
        }
    }
}

impl BuildXML for TableLayout {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.table_layout(&self.layout_type.to_string()).build()
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
