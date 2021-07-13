use serde::Serialize;

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
    fn build(&self) -> Vec<u8> {
        let mut b = XMLBuilder::new();
        b = b.declaration(Some(false));
        b = b
            .open_data_store_item(
                "http://schemas.openxmlformats.org/officeDocument/2006/customXml",
                &format!("{{{}}}", self.id),
            )
            .open_data_store_schema_refs()
            .close();
        b.close().build()
    }
}
