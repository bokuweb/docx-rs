#[macro_use]
mod macros;

mod comments;
mod comments_extended;
mod core_properties;
mod custom_properties;
mod declaration;
mod document;
mod drawing;
mod elements;
mod fonts;
mod footer;
mod footnotes;
mod header;
mod numbering;
mod pic;
mod properties;
mod relationship;
mod settings;

mod styles;

use crate::BuildXML;

use std::str;
use xml::common::XmlVersion;
use xml::writer::{EmitterConfig, EventWriter, XmlEvent};

pub use elements::*;

pub struct XMLBuilder {
    writer: EventWriter<Vec<u8>>,
}

impl XMLBuilder {
    pub(crate) fn new() -> XMLBuilder {
        let buf = Vec::new();
        let mut config = EmitterConfig::new()
            .write_document_declaration(false)
            .perform_indent(true);
        config.perform_escaping = false;
        let writer = config.create_writer(buf);
        XMLBuilder { writer }
    }

    // Build types element
    // i.e. <Types xmlns="http://...">
    pub(crate) fn open_types(mut self, uri: &str) -> Self {
        self.writer
            .write(XmlEvent::start_element("Types").attr("xmlns", uri))
            .expect("should write to buf");
        self
    }

    // Build Override element
    // i.e. <Override PartName="/_rels/.rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
    pub(crate) fn add_override(mut self, name: &str, content_type: &str) -> Self {
        self.writer
            .write(
                XmlEvent::start_element("Override")
                    .attr("PartName", name)
                    .attr("ContentType", content_type),
            )
            .expect("should write to buf");
        self.close()
    }

    pub(crate) fn add_default(mut self, name: &str, extension: &str) -> Self {
        self.writer
            .write(
                XmlEvent::start_element("Default")
                    .attr("ContentType", extension)
                    .attr("Extension", name),
            )
            .expect("should write to buf");
        self.close()
    }

    pub(crate) fn add_child<T>(mut self, child: &T) -> Self
    where
        T: BuildXML,
    {
        let buf = child.build();
        let text = str::from_utf8(&buf).unwrap();
        self.writer.write(text).expect("should write to buf");
        self
    }

    pub(crate) fn add_bytes(mut self, child: &[u8]) -> Self {
        let text = str::from_utf8(child).unwrap();
        self.writer.write(text).expect("should write to buf");
        self
    }

    pub(crate) fn add_optional_child<T>(mut self, child: &Option<T>) -> Self
    where
        T: BuildXML,
    {
        if let Some(c) = child {
            self = self.add_child(c)
        }
        self
    }

    pub(crate) fn add_children<T>(mut self, children: &[T]) -> Self
    where
        T: BuildXML,
    {
        for c in children {
            self = self.add_child(c);
        }
        self
    }

    // Close tag
    pub(crate) fn close(mut self) -> Self {
        self.writer
            .write(XmlEvent::end_element())
            .expect("should end");
        self
    }

    // Write plain text
    #[allow(dead_code)]
    pub(crate) fn plain_text(mut self, t: &str) -> Self {
        self.writer.write(t).unwrap();
        self
    }

    pub(crate) fn build(self) -> Vec<u8> {
        self.writer.into_inner()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_types() {
        let b = XMLBuilder::new();
        let r = b
            .open_types("http://example")
            .plain_text("child")
            .close()
            .build();
        assert_eq!(
            str::from_utf8(&r).unwrap(),
            r#"<Types xmlns="http://example">child</Types>"#
        );
    }
}
