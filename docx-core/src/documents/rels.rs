use crate::documents::BuildXML;
use crate::xml_builder::*;

pub struct Rels {}

impl Rels {
    pub fn new() -> Rels {
        Rels {}
    }
}

impl BuildXML for Rels {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.declaration(None)
            .open_relationships("http://schemas.openxmlformats.org/package/2006/relationships")
            .relationship(
                "rId1",
                "http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties",
                "docProps/core.xml"
            )
            .relationship(
                "rId2",
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships/extended-properties",
                "docProps/app.xml"
            ) 
            .relationship(
                "rId3",
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument",
             "word/document.xml"
            )                         
            .close()
            .build()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::str;

    #[test]
    fn test_build() {
        let c = Rels::new();
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<?xml version="1.0" encoding="UTF-8"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties" Target="docProps/core.xml" />
  <Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/extended-properties" Target="docProps/app.xml" />
  <Relationship Id="rId3" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="word/document.xml" />
</Relationships>"#
        );
    }
}
