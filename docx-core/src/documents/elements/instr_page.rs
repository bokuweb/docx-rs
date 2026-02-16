use serde::Serialize;
use std::io::Write;

use crate::documents::*;
use crate::xml_builder::XMLBuilder;

#[derive(Serialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct InstrPAGE {}

impl InstrPAGE {
    pub fn new() -> Self {
        Self {}
    }
}

impl BuildXML for InstrPAGE {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream).plain_text("PAGE")?.into_inner()
    }
}
