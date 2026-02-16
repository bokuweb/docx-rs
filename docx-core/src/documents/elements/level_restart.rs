use serde::{Serialize, Serializer};
use std::io::Write;

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, PartialEq)]
pub struct LevelRestart {
    val: u32,
}

impl LevelRestart {
    pub fn new(val: impl Into<u32>) -> Self {
        Self { val: val.into() }
    }
}

impl BuildXML for LevelRestart {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .level_restart(&format!("{}", &self.val))?
            .into_inner()
    }
}

impl Serialize for LevelRestart {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u32(self.val)
    }
}
