use serde::{Deserialize, Serialize, Serializer};
use std::io::Write;

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Bold {
    pub val: bool,
}

impl Bold {
    pub fn new() -> Bold {
        Default::default()
    }

    pub fn disable(mut self) -> Bold {
        self.val = false;
        self
    }
}

impl Default for Bold {
    fn default() -> Self {
        Self { val: true }
    }
}

impl Serialize for Bold {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bool(self.val)
    }
}

impl BuildXML for Bold {
    fn build_to<W: Write>(
        &self,
        stream: xml::writer::EventWriter<W>,
    ) -> xml::writer::Result<xml::writer::EventWriter<W>> {
        if self.val {
            XMLBuilder::from(stream).b()?.into_inner()
        } else {
            XMLBuilder::from(stream).disable_bold()?.into_inner()
        }
    }
}
