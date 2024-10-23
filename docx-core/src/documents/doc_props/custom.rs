use serde::Serialize;
use std::io::Write;

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CustomProps {
    pub properties: std::collections::HashMap<String, String>,
}

impl CustomProps {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub fn add_custom_property(mut self, name: impl Into<String>, item: impl Into<String>) -> Self {
        self.properties.insert(name.into(), item.into());
        self
    }
}

impl BuildXML for CustomProps {
    fn build_to<W: Write>(
        &self,
        stream: xml::writer::EventWriter<W>,
    ) -> xml::writer::Result<xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .declaration(Some(true))?
            .open_custom_properties(
                "http://schemas.openxmlformats.org/officeDocument/2006/custom-properties",
                "http://schemas.openxmlformats.org/officeDocument/2006/docPropsVTypes",
            )?
            .apply_each(self.properties.iter().enumerate(), |(i, (key, item)), b| {
                b.open_property(
                    "{D5CDD505-2E9C-101B-9397-08002B2CF9AE}",
                    // I can not find spec about this id.
                    // It is invalid if pid starts from 1...
                    &format!("{}", i + 2),
                    key,
                )?
                .lpwstr(item)?
                .close()
            })?
            .close()?
            .into_inner()
    }
}
