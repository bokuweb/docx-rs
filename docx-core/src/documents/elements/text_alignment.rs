use serde::{Serialize, Serializer};

use crate::documents::BuildXML;
use crate::{xml_builder::*, TextAlignmentType};

#[derive(Debug, Clone, PartialEq)]
pub struct TextAlignment(pub TextAlignmentType);

impl TextAlignment {
    pub fn new(val: TextAlignmentType) -> TextAlignment {
        TextAlignment(val)
    }
}

impl BuildXML for TextAlignment {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        let v = format!("{}", self.0);
        b.text_alignment(&v).build()
    }
}

impl Serialize for TextAlignment {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let v = format!("{}", self.0);
        serializer.serialize_str(&v)
    }
}
