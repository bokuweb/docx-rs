use serde::ser::{Serialize, SerializeStruct, Serializer};
use serde::Deserialize;

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Sym {
    pub font: String,
    pub char: String,
}

impl Sym {
    pub fn new(font: impl Into<String>, char: impl Into<String>) -> Self {
        Self {
            font: font.into(),
            char: char.into(),
        }
    }
}
impl BuildXML for Sym {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.sym(&self.font, &self.char).build()
    }
}

impl Serialize for Sym {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut t = serializer.serialize_struct("Sym", 1)?;
        t.serialize_field("font", &self.font)?;
        t.serialize_field("char", &self.char)?;
        t.end()
    }
}
