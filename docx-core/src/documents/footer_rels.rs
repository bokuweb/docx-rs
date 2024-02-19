use crate::documents::BuildXML;
use crate::{xml_builder::*, ImageIdAndPath};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FooterRels {
    pub images: Vec<(String, String)>,
}

impl FooterRels {
    pub fn new() -> FooterRels {
        Default::default()
    }

    pub fn add_image(mut self, id: impl Into<String>, path: impl Into<String>) -> Self {
        self.images.push((id.into(), path.into()));
        self
    }

    pub(crate) fn set_images(&mut self, images: Vec<ImageIdAndPath>) {
        self.images = images;
    }
}

impl BuildXML for FooterRels {
    fn build(&self) -> Vec<u8> {
        let mut b = XMLBuilder::new();
        b = b
            .declaration(None)
            .open_relationships("http://schemas.openxmlformats.org/package/2006/relationships");

        for (id, path) in self.images.iter() {
            b = b.relationship(
                id,
                "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image",
                path,
            )
        }

        b.close().build()
    }
}
