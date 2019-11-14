use super::XMLBuilder;
use super::XmlEvent;

impl XMLBuilder {
    pub(crate) fn open_fonts(mut self) -> Self {
        self.writer
            .write(
                XmlEvent::start_element("w:fonts")
                    .attr(
                        "xmlns:w",
                        "http://schemas.openxmlformats.org/wordprocessingml/2006/main",
                    )
                    .attr(
                        "xmlns:r",
                        "http://schemas.openxmlformats.org/officeDocument/2006/relationships",
                    ),
            )
            .expect("should write to buf");
        self
    }
}
