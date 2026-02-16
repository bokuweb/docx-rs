use serde::Serialize;
use std::io::Write;

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CustomItemRels {
    custom_item_count: usize,
}

impl CustomItemRels {
    pub fn new() -> CustomItemRels {
        Default::default()
    }

    pub fn add_item(mut self) -> Self {
        self.custom_item_count += 1;
        self
    }
}

impl BuildXML for CustomItemRels {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .declaration(Some(true))?
            .open_relationships("http://schemas.openxmlformats.org/package/2006/relationships")?
            .apply_each(0..self.custom_item_count, |id, b| {
                b.relationship(
          &format!("rId{}", id + 1),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXmlProps",
          &format!("itemProps{}.xml", id + 1),
        )
            })?
            .close()?
            .into_inner()
    }
}
