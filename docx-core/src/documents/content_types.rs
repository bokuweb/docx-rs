use crate::xml_builder::*;

pub struct ContentTypes {}

impl ContentTypes {
    pub fn new() -> ContentTypes {
        ContentTypes {}
    }

    pub fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.declaration()
            .open_types("http://schemas.openxmlformats.org/package/2006/content-types")
            .add_override(
                "/_rels/.rels",
                "application/vnd.openxmlformats-package.relationships+xml",
            )
            .add_override(
                "/docProps/app.xml",
                "application/vnd.openxmlformats-officedocument.extended-properties+xml",
            )
            .add_override(
                "/docProps/core.xml",
                "application/vnd.openxmlformats-package.core-properties+xml",
            )
            .add_override(
                "/word/_rels/document.xml.rels",
                "application/vnd.openxmlformats-package.relationships+xml",
            )
            .add_override(
                "/word/settings.xml",
                "application/vnd.openxmlformats-officedocument.wordprocessingml.settings+xml",
            )
            .add_override(
                "/word/fontTable.xml",
                "application/vnd.openxmlformats-officedocument.wordprocessingml.fontTable+xml",
            )
            .add_override(
                "/word/document.xml",
                "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml",
            )
            .add_override(
                "/word/styles.xml",
                "application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml",
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
        let c = ContentTypes::new();
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<?xml version="1.0" encoding="UTF-8"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
  <Override PartName="/_rels/.rels" ContentType="application/vnd.openxmlformats-package.relationships+xml" />
  <Override PartName="/docProps/app.xml" ContentType="application/vnd.openxmlformats-officedocument.extended-properties+xml" />
  <Override PartName="/docProps/core.xml" ContentType="application/vnd.openxmlformats-package.core-properties+xml" />
  <Override PartName="/word/_rels/document.xml.rels" ContentType="application/vnd.openxmlformats-package.relationships+xml" />
  <Override PartName="/word/settings.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.settings+xml" />
  <Override PartName="/word/fontTable.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.fontTable+xml" />
  <Override PartName="/word/document.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml" />
  <Override PartName="/word/styles.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml" />
</Types>"#
        );
    }
}
