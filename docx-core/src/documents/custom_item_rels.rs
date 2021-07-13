use serde::Serialize;

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
    fn build(&self) -> Vec<u8> {
        let mut b = XMLBuilder::new();
        b = b
            .declaration(Some(true))
            .open_relationships("http://schemas.openxmlformats.org/package/2006/relationships");

        for id in 0..self.custom_item_count {
            let id = id + 1;
            b = b.relationship(
                &format!("rId{}", id),
                "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXmlProps",
                &format!("itemProps{}.xml", id),
            )
        }

        b.close().build()
    }
}
