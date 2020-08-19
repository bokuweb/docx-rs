use super::XMLBuilder;
use super::XmlEvent;

impl XMLBuilder {
    pub(crate) fn open_settings(mut self) -> Self {
        self.writer
            .write(
                XmlEvent::start_element("w:settings")
                    .attr(
                        "xmlns:w",
                        "http://schemas.openxmlformats.org/wordprocessingml/2006/main",
                    )
                    .attr(
                        "xmlns:w14",
                        "http://schemas.microsoft.com/office/word/2010/wordml",
                    )
                    .attr(
                        "xmlns:w15",
                        "http://schemas.microsoft.com/office/word/2012/wordml",
                    ),
            )
            .expect("should write to buf");
        self
    }
}
