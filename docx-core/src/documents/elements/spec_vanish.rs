use serde::{Deserialize, Serialize, Serializer};

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, Deserialize, PartialEq, Default)]
pub struct SpecVanish {}

impl SpecVanish {
    pub fn new() -> SpecVanish {
        SpecVanish {}
    }
}

impl BuildXML for SpecVanish {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.spec_vanish().build()
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
