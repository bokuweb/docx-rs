use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug)]
pub struct ContentTypes {}

impl ContentTypes {
    pub fn new() -> ContentTypes {
        ContentTypes {}
    }
}

impl BuildXML for ContentTypes {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.declaration(None)
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
            .add_override(
                "/word/comments.xml",
                "application/vnd.openxmlformats-officedocument.wordprocessingml.comments+xml",
            )
            .add_override(
                "/word/numbering.xml",
                "application/vnd.openxmlformats-officedocument.wordprocessingml.numbering+xml",
            )
            .close()
            .build()
    }
}
