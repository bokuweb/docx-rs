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
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        // Ignore for now
        Ok(stream)
    }
}
