use serde::{Deserialize, Serialize, Serializer};

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Italic {}

impl Italic {
    pub fn new() -> Italic {
        Default::default()
    }
}

impl Default for Italic {
    fn default() -> Self {
        Self {}
    }
}

impl Serialize for Italic {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bool(true)
    }
}

impl BuildXML for Italic {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.i().build()
    }
}
