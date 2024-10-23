use super::XMLBuilder;
use super::XmlEvent;

use std::io::Write;
use xml::writer::Result;

impl<W: Write> XMLBuilder<W> {
    pub(crate) fn open_fonts(self) -> Result<Self> {
        self.write(
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
    }
}
