use serde::{Deserialize, Serialize, Serializer};

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[cfg_attr(feature = "wasm", derive(ts_rs::TS))]
#[cfg_attr(feature = "wasm", ts(export))]
pub struct BoldCs(pub bool);

impl BoldCs {
    pub fn new() -> BoldCs {
        Default::default()
    }
    pub fn disable(mut self) -> BoldCs {
        self.0 = false;
        self
    }
}

impl Default for BoldCs {
    fn default() -> Self {
        Self(true)
    }
}

impl Serialize for BoldCs {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bool(self.0)
    }
}

impl BuildXML for BoldCs {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.b_cs().build()
    }
}
