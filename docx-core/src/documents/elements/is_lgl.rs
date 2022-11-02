use serde::{Deserialize, Serialize, Serializer};

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct IsLgl {}

impl IsLgl {
    pub fn new() -> IsLgl {
        IsLgl {}
    }
}

impl Default for IsLgl {
    fn default() -> Self {
        IsLgl {}
    }
}

impl BuildXML for IsLgl {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.is_lgl().build()
    }
}

impl Serialize for IsLgl {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bool(true)
    }
}
