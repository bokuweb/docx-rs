use serde::{Deserialize, Serialize, Serializer};

// use crate::documents::BuildXML;
// use crate::xml_builder::*;

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[cfg_attr(feature = "wasm", derive(ts_rs::TS))]
#[cfg_attr(feature = "wasm", ts(export))]
pub struct Caps(pub bool);

impl Caps {
    pub fn new() -> Caps {
        Default::default()
    }

    pub fn disable(mut self) -> Caps {
        self.0 = false;
        self
    }
}

impl Default for Caps {
    fn default() -> Self {
        Self(true)
    }
}

impl Serialize for Caps {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bool(self.0)
    }
}
