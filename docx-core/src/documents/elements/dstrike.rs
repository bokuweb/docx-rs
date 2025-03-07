use serde::{Deserialize, Serialize, Serializer};
use std::io::Write;

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Dstrike {
    pub val: bool,
}

impl Dstrike {
    pub fn new() -> Dstrike {
        Default::default()
    }

    pub fn disable(mut self) -> Dstrike {
        self.val = false;
        self
    }
}

impl Default for Dstrike {
    fn default() -> Self {
        Self { val: true }
    }
}

impl Serialize for Dstrike {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bool(self.val)
    }
}

impl BuildXML for Dstrike {
    fn build_to<W: Write>(
        &self,
        stream: xml::writer::EventWriter<W>,
    ) -> xml::writer::Result<xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream).dstrike()?.into_inner()
    }
}
