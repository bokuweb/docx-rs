use serde::{Deserialize, Serialize, Serializer};

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct BoldCs {}

impl BoldCs {
    pub fn new() -> BoldCs {
        Default::default()
    }
}

impl Default for BoldCs {
    fn default() -> Self {
        Self {}
    }
}

impl Serialize for BoldCs {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bool(true)
    }
}

impl BuildXML for BoldCs {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.b_cs().build()
    }
}
