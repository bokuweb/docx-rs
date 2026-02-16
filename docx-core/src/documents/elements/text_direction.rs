use serde::{Serialize, Serializer};
use std::io::Write;

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
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .text_direction(&self.val.to_string())?
            .into_inner()
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
