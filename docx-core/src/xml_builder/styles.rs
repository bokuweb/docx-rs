use super::XMLBuilder;
use super::XmlEvent;
use std::io::Write;
use xml::writer::Result;

impl<W: Write> XMLBuilder<W> {
    // Build w:style element
    // i.e. <w:styles ... >
    pub(crate) fn open_styles(self) -> Result<Self> {
        self.write(
            XmlEvent::start_element("w:styles")
                .attr(
                    "xmlns:mc",
                    "http://schemas.openxmlformats.org/markup-compatibility/2006",
                )
                .attr(
                    "xmlns:r",
                    "http://schemas.openxmlformats.org/officeDocument/2006/relationships",
                )
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
                )
                .attr("mc:Ignorable", "w14 w15"),
        )
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_declaration() -> Result<()> {
        let b = XMLBuilder::new(Vec::new());
        let r = b.open_styles()?.close()?.into_inner()?.into_inner();
        assert_eq!(
            str::from_utf8(&r).unwrap(),
            r#"<w:styles xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:w14="http://schemas.microsoft.com/office/word/2010/wordml" xmlns:w15="http://schemas.microsoft.com/office/word/2012/wordml" mc:Ignorable="w14 w15" />"#
        );
        Ok(())
    }
}
