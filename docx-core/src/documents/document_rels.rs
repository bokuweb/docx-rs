use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug)]
pub struct DocumentRels {}

impl DocumentRels {
    pub fn new() -> DocumentRels {
        DocumentRels {}
    }
}

impl BuildXML for DocumentRels {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.declaration(None)
            .open_relationships("http://schemas.openxmlformats.org/package/2006/relationships")
            .relationship(
                "rId1",
                "http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles",
                "styles.xml",
            )
            .relationship(
                "rId2",
                "http://schemas.openxmlformats.org/officeDocument/2006/relationships/numbering",
                "numbering.xml",
            )
            .relationship(
                "rId3",
                "http://schemas.openxmlformats.org/officeDocument/2006/relationships/comments",
                "comments.xml",
            )
            .relationship(
                "rId4",
                "http://schemas.openxmlformats.org/officeDocument/2006/relationships/fontTable",
                "fontTable.xml",
            )
            .relationship(
                "rId5",
                "http://schemas.openxmlformats.org/officeDocument/2006/relationships/settings",
                "settings.xml",
            )
            .close()
            .build()
    }
}
