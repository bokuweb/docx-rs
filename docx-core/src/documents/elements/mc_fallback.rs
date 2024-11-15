use crate::documents::BuildXML;
use serde::Serialize;
use std::io::Write;

#[derive(Debug, Default, Clone, PartialEq, Serialize)]
pub struct McFallback {}

impl McFallback {
    pub fn new() -> McFallback {
        Default::default()
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
