use serde::{Deserialize, Serialize, Serializer};
use std::io::Write;

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Italic {
    val: bool,
}

impl Italic {
    pub fn new() -> Italic {
        Default::default()
    }

    pub fn disable(mut self) -> Self {
        self.val = false;
        self
    }
}

impl Default for Italic {
    fn default() -> Self {
        Self { val: true }
    }
}

impl Serialize for Italic {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bool(self.val)
    }
}

impl BuildXML for Italic {
    fn build_to<W: Write>(
        &self,
        stream: xml::writer::EventWriter<W>,
    ) -> xml::writer::Result<xml::writer::EventWriter<W>> {
        if self.val {
            XMLBuilder::from(stream).i()?.into_inner()
        } else {
            XMLBuilder::from(stream).disable_italic()?.into_inner()
        }
    }
}
