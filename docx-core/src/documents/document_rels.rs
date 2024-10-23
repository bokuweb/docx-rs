use serde::Serialize;
use std::io::Write;

use super::*;
use crate::documents::BuildXML;
use crate::{escape::*, xml_builder::*};

#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DocumentRels {
    pub has_comments: bool,
    pub has_numberings: bool,
    pub has_footnotes: bool,
    pub images: Vec<(String, String)>,
    pub hyperlinks: Vec<(String, String, String)>,
    pub custom_xml_count: usize,
    pub header_count: usize,
    pub footer_count: usize,
}

impl DocumentRels {
    pub fn new() -> DocumentRels {
        Default::default()
    }

    pub fn add_custom_item(mut self) -> Self {
        self.custom_xml_count += 1;
        self
    }

    pub fn add_image(mut self, id: impl Into<String>, path: impl Into<String>) -> Self {
        self.images.push((id.into(), path.into()));
        self
    }

    pub fn add_hyperlinks(
        mut self,
        id: impl Into<String>,
        path: impl Into<String>,
        r#type: impl Into<String>,
    ) -> Self {
        self.hyperlinks
            .push((id.into(), escape(&path.into()), r#type.into()));
        self
    }
}

impl BuildXML for DocumentRels {
    fn build_to<W: Write>(
        &self,
        stream: xml::writer::EventWriter<W>,
    ) -> xml::writer::Result<xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .declaration(None)?
            .open_relationships("http://schemas.openxmlformats.org/package/2006/relationships")?
            .relationship(
                "rId1",
                "http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles",
                "styles.xml",
            )?
            .relationship(
                "rId2",
                "http://schemas.openxmlformats.org/officeDocument/2006/relationships/fontTable",
                "fontTable.xml",
            )?
            .relationship(
                "rId3",
                "http://schemas.openxmlformats.org/officeDocument/2006/relationships/settings",
                "settings.xml",
            )?
            .relationship(
                "rId5",
                "http://schemas.microsoft.com/office/2011/relationships/commentsExtended",
                "commentsExtended.xml",
            )?
            .apply_if(self.has_comments, |b| {
                b.relationship(
                    "rId6",
                    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/comments",
                    "comments.xml",
                )
            })?
            .apply_if(self.has_numberings, |b| {
                b.relationship(
                    "rId7",
                    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/numbering",
                    "numbering.xml",
                )
            })?
            .apply_if(self.has_footnotes, |b| {
                b.relationship(
                    "rId8",
                    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/footnotes",
                    "footnotes.xml",
                )
            })?
            .apply_each(0..self.header_count, |i, b| {
                b.relationship(
                    &create_header_rid(i + 1),
                    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/header",
                    &format!("header{}.xml", i + 1),
                )
            })?
            .apply_each(0..self.footer_count, |i, b| {
                b.relationship(
                    &create_footer_rid(i + 1),
                    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/footer",
                    &format!("footer{}.xml", i + 1),
                )
            })?
            .apply_each(0..self.custom_xml_count, |i, b| {
                b.relationship(
                    &format!("rId{}", i + 8),
                    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXml",
                    &format!("../customXml/item{}.xml", i + 1),
                )
            })?
            .apply_each(self.images.iter(), |(id, path), b| {
                b.relationship(
                    id,
                    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image",
                    path,
                )
            })?
            .apply_each(self.hyperlinks.iter(), |(id, path, r#type), b| {
                b.relationship_with_mode(
                    id,
                    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/hyperlink",
                    path,
                    r#type,
                )
            })?
            .close()?
            .into_inner()
    }
}
