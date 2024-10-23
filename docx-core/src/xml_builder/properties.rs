use super::XMLBuilder;
use super::XmlEvent;

use std::io::Write;

impl<W: Write> XMLBuilder<W> {
    // Build Properties element
    // i.e. <Properties xmlns:vt="http://schemas.openxmlformats.org/package/2006/relationships">
    open!(open_properties, "Properties", "xmlns", "xmlns:vt");

    closed_with_child!(template, "Template");
    closed_with_child!(total_time, "TotalTime");
    closed_with_child!(application, "Application");
    closed_with_child!(pages, "Pages");
    closed_with_child!(words, "Words");
    closed_with_child!(characters, "Characters");
    closed_with_child!(characters_with_spaces, "CharactersWithSpaces");
    closed_with_child!(paragraphs, "Paragraphs");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str;
    use xml::writer::Result;

    #[test]
    fn test_properties() -> Result<()> {
        let b = XMLBuilder::new(Vec::new());
        let r = b
            .open_properties("http://example", "http://example2")?
            .plain_text("child")?
            .close()?
            .into_inner()?
            .into_inner();
        assert_eq!(
            str::from_utf8(&r).unwrap(),
            r#"<Properties xmlns="http://example" xmlns:vt="http://example2">child</Properties>"#
        );
        Ok(())
    }

    #[test]
    fn test_template() -> Result<()> {
        let b = XMLBuilder::new(Vec::new());
        let r = b.template("0")?.into_inner()?.into_inner();
        assert_eq!(str::from_utf8(&r).unwrap(), r#"<Template>0</Template>"#);
        Ok(())
    }

    #[test]
    fn test_application() -> Result<()> {
        let b = XMLBuilder::new(Vec::new());
        let r = b.application("Lawgue")?.into_inner()?.into_inner();
        assert_eq!(
            str::from_utf8(&r).unwrap(),
            r#"<Application>Lawgue</Application>"#
        );
        Ok(())
    }
}
