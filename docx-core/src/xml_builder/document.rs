use super::XMLBuilder;
use super::XmlEvent;

impl XMLBuilder {
    // i.e. <w:document ... >
    pub(crate) fn open_document(mut self) -> Self {
        self.writer
            .write(
                XmlEvent::start_element("w:document")
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
                    .attr(
                        "xmlns:w15",
                        "http://schemas.microsoft.com/office/word/2012/wordml",
                    )
                    .attr("mc:Ignorable", "w14 wp14"),
            )
            .expect("should write to buf");
        self
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_document() {
        let b = XMLBuilder::new();
        let r = b.open_document().close().build();
        assert_eq!(
            str::from_utf8(&r).unwrap(),
            r#"<w:document xmlns:o="urn:schemas-microsoft-com:office:office" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:v="urn:schemas-microsoft-com:vml" xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:w10="urn:schemas-microsoft-com:office:word" xmlns:wp="http://schemas.openxmlformats.org/drawingml/2006/wordprocessingDrawing" xmlns:wps="http://schemas.microsoft.com/office/word/2010/wordprocessingShape" xmlns:wpg="http://schemas.microsoft.com/office/word/2010/wordprocessingGroup" xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006" xmlns:wp14="http://schemas.microsoft.com/office/word/2010/wordprocessingDrawing" xmlns:w14="http://schemas.microsoft.com/office/word/2010/wordml" xmlns:w15="http://schemas.microsoft.com/office/word/2012/wordml" mc:Ignorable="w14 wp14" />"#
        );
    }
}
