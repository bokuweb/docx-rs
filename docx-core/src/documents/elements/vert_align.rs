use serde::{Serialize, Serializer};

use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;

#[derive(Debug, Clone, PartialEq)]
pub struct VertAlign {
    val: VertAlignType,
}

impl VertAlign {
    pub fn new(val: VertAlignType) -> VertAlign {
        Self { val }
    }
}

impl Serialize for VertAlign {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{}", &self.val))
    }
}

impl BuildXML for VertAlign {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.vert_align(&self.val.to_string() ).build()
    }
}
