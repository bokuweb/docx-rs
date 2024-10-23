use std::io::Write;
// use super::*;
use serde::Serialize;

use crate::documents::BuildXML;
// use crate::xml_builder::*;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct McFallback {}

impl McFallback {
    pub fn new() -> McFallback {
        Default::default()
    }
}

impl Default for McFallback {
    fn default() -> Self {
        McFallback {}
    }
}

impl BuildXML for McFallback {
    fn build_to<W: Write>(
        &self,
        stream: xml::writer::EventWriter<W>,
    ) -> xml::writer::Result<xml::writer::EventWriter<W>> {
        // Ignore for now
        Ok(stream)
    }
}
