use serde::{Deserialize, Serialize, Serializer};

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, Deserialize, PartialEq, Default)]
pub struct IsLgl {}

impl IsLgl {
    pub fn new() -> IsLgl {
        IsLgl {}
    }
}

impl BuildXML for IsLgl {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new(Vec::new());
        b.is_lgl().into_inner()
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
