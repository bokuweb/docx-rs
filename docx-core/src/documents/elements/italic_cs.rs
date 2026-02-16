use serde::{Deserialize, Serialize, Serializer};
use std::io::Write;

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct ItalicCs {
    val: bool,
}

impl ItalicCs {
    pub fn new() -> ItalicCs {
        Default::default()
    }

    pub fn disable(mut self) -> Self {
        self.val = false;
        self
    }
}

impl Default for ItalicCs {
    fn default() -> Self {
        Self { val: true }
    }
}

impl Serialize for ItalicCs {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bool(self.val)
    }
}

impl BuildXML for ItalicCs {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream).i_cs()?.into_inner()
    }
}
