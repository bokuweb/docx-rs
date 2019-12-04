use super::XMLBuilder;
use super::XmlEvent;

impl XMLBuilder {
    pub(crate) fn open_comments(mut self) -> Self {
        self.writer
            .write(
                XmlEvent::start_element("w:comments")
                    .attr("xmlns:o", "urn:schemas-microsoft-com:office:office")
                    .attr(
                        "xmlns:r",
                        "http://schemas.openxmlformats.org/officeDocument/2006/relationships",
                    )
                    .attr("xmlns:v", "urn:schemas-microsoft-com:vml")
                    .attr(
                        "xmlns:w",
                        "http://schemas.openxmlformats.org/wordprocessingml/2006/main",
                    )
                    .attr("xmlns:w10", "urn:schemas-microsoft-com:office:word")
                    .attr(
                        "xmlns:wp",
                        "http://schemas.openxmlformats.org/drawingml/2006/wordprocessingDrawing",
                    )
                    .attr(
                        "xmlns:wps",
                        "http://schemas.microsoft.com/office/word/2010/wordprocessingShape",
                    )
                    .attr(
                        "xmlns:wpg",
                        "http://schemas.microsoft.com/office/word/2010/wordprocessingGroup",
                    )
                    .attr(
                        "xmlns:mc",
                        "http://schemas.openxmlformats.org/markup-compatibility/2006",
                    )
                    .attr(
                        "xmlns:wp14",
                        "http://schemas.microsoft.com/office/word/2010/wordprocessingDrawing",
                    )
                    .attr(
                        "xmlns:w14",
                        "http://schemas.microsoft.com/office/word/2010/wordml",
                    )
                    .attr("mc:Ignorable", "w14 wp14"),
            )
            .expect("should write to buf");
        self
    }
}
