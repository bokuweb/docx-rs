use serde::{Serialize, Serializer};
use std::io::Write;

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
    fn build_to<W: Write>(
        &self,
        stream: xml::writer::EventWriter<W>,
    ) -> xml::writer::Result<xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .text_alignment(&format!("{}", self.0))?
            .into_inner()
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
