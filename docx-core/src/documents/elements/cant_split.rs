use serde::{Serialize, Serializer};

use crate::{xml_builder::XMLBuilder, BuildXML};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CantSplit {}
impl BuildXML for CantSplit {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.cant_split().build()
    }
}

impl Serialize for CantSplit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("cantSplit")
    }
}
