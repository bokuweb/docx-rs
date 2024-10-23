use crate::documents::BuildXML;
use crate::xml_builder::*;
use std::io::Write;

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FooterReference {
    pub footer_type: String,
    pub id: String,
}

impl FooterReference {
    pub fn new(t: impl Into<String>, id: impl Into<String>) -> FooterReference {
        FooterReference {
            footer_type: t.into(),
            id: id.into(),
        }
    }
}

impl BuildXML for FooterReference {
    fn build_to<W: Write>(
        &self,
        stream: xml::writer::EventWriter<W>,
    ) -> xml::writer::Result<xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .footer_reference(&self.footer_type, &self.id)?
            .into_inner()
    }
}
