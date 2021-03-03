use serde::{Serialize, Serializer};

use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;

#[derive(Debug, Clone, PartialEq)]
pub struct TextDirection {
    val: TextDirectionType,
}

impl TextDirection {
    pub fn new(t: TextDirectionType) -> TextDirection {
        TextDirection { val: t }
    }
}

impl BuildXML for TextDirection {
    fn build(&self) -> Vec<u8> {
        XMLBuilder::new()
            .text_direction(&self.val.to_string())
            .build()
    }
}

impl Serialize for TextDirection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{}", &self.val))
    }
}
