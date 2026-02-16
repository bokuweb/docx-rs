use serde::Serialize;
use std::io::Write;

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, Serialize)]
pub struct CustomItemProperty {
    id: String,
}

impl CustomItemProperty {
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into() }
    }
}

impl BuildXML for CustomItemProperty {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .declaration(Some(false))?
            .open_data_store_item(
                "http://schemas.openxmlformats.org/officeDocument/2006/customXml",
                &format!("{{{}}}", self.id),
            )?
            .open_data_store_schema_refs()?
            .close()?
            .close()?
            .into_inner()
    }
}
