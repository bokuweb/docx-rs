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
                "http://schemas.openxmlformats.org/officeDocument/2006/relationships/fontTable",
                "fontTable.xml",
            )
            .relationship(
                "rId3",
                "http://schemas.openxmlformats.org/officeDocument/2006/relationships/settings",
                "settings.xml",
            )
            .relationship(
                "rId4",
                "http://schemas.openxmlformats.org/officeDocument/2006/relationships/tag",
                "tag.xml",
            )
            .close()
            .build()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_build() {
        let c = DocumentRels::new();
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<?xml version="1.0" encoding="UTF-8"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles" Target="styles.xml" />
  <Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/fontTable" Target="fontTable.xml" />
  <Relationship Id="rId3" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/settings" Target="settings.xml" />
  <Relationship Id="rId4" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/tag" Target="tag.xml" />
</Relationships>"#
        );
    }
}
