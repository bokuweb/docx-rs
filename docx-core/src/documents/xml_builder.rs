use xml::writer::{EmitterConfig, EventWriter, XmlEvent};

pub struct XMLBuilder {
    writer: EventWriter<Vec<u8>>,
}

impl XMLBuilder {
    pub(crate) fn new() -> XMLBuilder {
        let buf = Vec::new();
        let writer = EmitterConfig::new()
            .write_document_declaration(false)
            .perform_indent(true)
            .create_writer(buf);
        XMLBuilder { writer }
    }

    pub(crate) fn add_declaration(mut self) -> Self {
        self.writer
            .write(
                XmlEvent::start_element("?xml")
                    .attr("version", "1.0")
                    .attr("encoding", "UTF-8"),
            )
            .expect("should write to buf");
        self.close()
    }

    pub(crate) fn open_types(mut self, uri: &str) -> Self {
        self.writer
            .write(XmlEvent::start_element("Types").attr("xmlns", uri))
            .expect("should write to buf");
        self
    }

    pub(crate) fn add_override(mut self, name: &str, content_type: &str) -> Self {
        self.writer
            .write(
                XmlEvent::start_element("Override")
                    .attr("PartName", &name)
                    .attr("ContentType", &content_type),
            )
            .expect("should write to buf");
        self.close()
    }

    pub(crate) fn close(mut self) -> Self {
        self.writer
            .write(XmlEvent::end_element())
            .expect("should end");
        self
    }

    pub(crate) fn build(self) -> Vec<u8> {
        self.writer.into_inner()
    }
}
