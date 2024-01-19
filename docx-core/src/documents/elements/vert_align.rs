use serde::{Serialize, Serializer};

use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "wasm", derive(ts_rs::TS))]
#[cfg_attr(feature = "wasm", ts(export))]
pub struct VertAlign(
    #[cfg_attr(feature = "wasm", ts(type = "string"))] // TODO:
    VertAlignType,
);

impl VertAlign {
    pub fn new(val: VertAlignType) -> VertAlign {
        Self(val)
    }
}

impl Serialize for VertAlign {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{}", &self.0))
    }
}

impl BuildXML for VertAlign {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.vert_align(&self.0.to_string()).build()
    }
}
