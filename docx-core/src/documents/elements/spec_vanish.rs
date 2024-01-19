use serde::{Deserialize, Serialize, Serializer};

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[cfg_attr(feature = "wasm", derive(ts_rs::TS))]
#[cfg_attr(feature = "wasm", ts(export))]
pub struct SpecVanish(pub bool);

impl SpecVanish {
    pub fn new() -> SpecVanish {
        SpecVanish(true)
    }
}

impl Default for SpecVanish {
    fn default() -> Self {
        SpecVanish(true)
    }
}

impl BuildXML for SpecVanish {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.vanish().build()
    }
}

impl Serialize for SpecVanish {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bool(true)
    }
}
