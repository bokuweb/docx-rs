use serde::{Deserialize, Serialize, Serializer};

// use crate::documents::BuildXML;
// use crate::xml_builder::*;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Caps {
    val: bool,
}

impl Caps {
    pub fn new() -> Caps {
        Default::default()
    }

    pub fn disable(mut self) -> Caps {
        self.val = false;
        self
    }
}

impl Default for Caps {
    fn default() -> Self {
        Self { val: true }
    }
}

impl Serialize for Caps {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bool(self.val)
    }
}
