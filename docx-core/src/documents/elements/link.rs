use serde::{Serialize, Serializer};
use std::io::Write;

use crate::documents::BuildXML;
use crate::escape::escape;
use crate::xml_builder::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Link {
    val: String,
}

impl Link {
    pub fn new(val: impl Into<String>) -> Link {
        Link {
            val: escape(&val.into()),
        }
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
    fn build_to<W: Write>(
        &self,
        stream: xml::writer::EventWriter<W>,
    ) -> xml::writer::Result<xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream).link(&self.val)?.into_inner()
    }
}
