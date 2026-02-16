use crate::documents::BuildXML;
use crate::{xml_builder::*, ImageIdAndPath};
use serde::Serialize;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct HeaderRels {
    pub images: Vec<(String, String)>,
}

impl HeaderRels {
    pub fn new() -> HeaderRels {
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

impl BuildXML for HeaderRels {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .declaration(None)?
            .open_relationships("http://schemas.openxmlformats.org/package/2006/relationships")?
            .apply_each(&self.images, |(id, path), b| {
                b.relationship(
                    id,
                    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image",
                    path,
                )
            })?
            .close()?
            .into_inner()
    }
}
