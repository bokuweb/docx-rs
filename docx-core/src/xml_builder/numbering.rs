use super::XMLBuilder;
use super::XmlEvent;

use std::io::Write;
use xml::writer::Result;

impl<W: Write> XMLBuilder<W> {
    pub(crate) fn open_numbering(self) -> Result<Self> {
        self.write(
            XmlEvent::start_element("w:numbering")
                .attr(
                    "xmlns:r",
                    "http://schemas.openxmlformats.org/officeDocument/2006/relationships",
                )
                .attr("xmlns:o", "urn:schemas-microsoft-com:office:office")
                .attr("xmlns:v", "urn:schemas-microsoft-com:vml")
                .attr(
                    "xmlns:w",
                    "http://schemas.openxmlformats.org/wordprocessingml/2006/main",
                ),
        )
    }
}
