use serde::{Deserialize, Serialize, Serializer};
use std::io::Write;

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Strike {
    pub val: bool,
}

impl Strike {
    pub fn new() -> Strike {
        Default::default()
    }

    pub fn disable(mut self) -> Strike {
        self.val = false;
        self
    }
}

impl Default for Strike {
    fn default() -> Self {
        Self { val: true }
    }
}

impl Serialize for Strike {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bool(self.val)
    }
}

impl BuildXML for Strike {
    fn build_to<W: Write>(
        &self,
        stream: xml::writer::EventWriter<W>,
    ) -> xml::writer::Result<xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream).strike()?.into_inner()
    }
}
