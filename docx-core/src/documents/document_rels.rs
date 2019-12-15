use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug)]
pub struct DocumentRels {
    pub(crate) has_comments: bool,
    pub(crate) has_numberings: bool,
}

impl DocumentRels {
    pub fn new() -> DocumentRels {
        DocumentRels {
            has_comments: false,
            has_numberings: false,
        }
    }
}

impl BuildXML for DocumentRels {
    fn build(&self) -> Vec<u8> {
        let mut b = XMLBuilder::new();
        b = b
            .declaration(None)
            .open_relationships("http://schemas.openxmlformats.org/package/2006/relationships")
            .relationship(
                "rId1",
                "http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles",
                "styles.xml",
            )
            .relationship(
                "rId2",
                "http://schemas.openxmlformats.org/officeDocument/2006/relationships/fontTable",
                "fontTable.xml",
            )
            .relationship(
                "rId3",
                "http://schemas.openxmlformats.org/officeDocument/2006/relationships/settings",
                "settings.xml",
            );

        if self.has_comments {
            b = b.relationship(
                "rId4",
                "http://schemas.openxmlformats.org/officeDocument/2006/relationships/comments",
                "comments.xml",
            )
        }

        if self.has_numberings {
            b = b.relationship(
                "rId5",
                "http://schemas.openxmlformats.org/officeDocument/2006/relationships/numbering",
                "numbering.xml",
            )
        }

        b.close().build()
    }
}
