use serde::{Serialize, Serializer};

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Link {
    val: String,
}

impl Link {
    pub fn new(val: impl Into<String>) -> Link {
        Link { val: val.into() }
    }
}

impl Serialize for Link {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.val)
    }
}

impl BuildXML for Link {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.link(&self.val).build()
    }
}
