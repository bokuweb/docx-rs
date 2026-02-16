use super::XMLBuilder;
use super::XmlEvent;
use crate::xml::writer::Result;
use std::io::Write;

impl<W: Write> XMLBuilder<W> {
    pub(crate) fn open_settings(self) -> Result<Self> {
        self.write(
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
    }
}
