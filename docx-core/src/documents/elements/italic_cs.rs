use serde::{Deserialize, Serialize, Serializer};

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct ItalicCs {}

impl ItalicCs {
    pub fn new() -> ItalicCs {
        Default::default()
    }
}

impl Default for ItalicCs {
    fn default() -> Self {
        Self {}
    }
}

impl Serialize for ItalicCs {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bool(true)
    }
}

impl BuildXML for ItalicCs {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.i_cs().build()
    }
}
