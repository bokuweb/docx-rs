use super::XMLBuilder;
use super::XmlEvent;

impl XMLBuilder {
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

    use super::XMLBuilder;
    use std::str;

    #[test]
    fn test_properties() {
        let b = XMLBuilder::new();
        let r = b
            .open_properties("http://example", "http://example2")
            .plain_text("child")
            .close()
            .build();
        assert_eq!(
            str::from_utf8(&r).unwrap(),
            r#"<Properties xmlns="http://example" xmlns:vt="http://example2">child</Properties>"#
        );
    }

    #[test]
    fn test_template() {
        let b = XMLBuilder::new();
        let r = b.template("0").build();
        assert_eq!(str::from_utf8(&r).unwrap(), r#"<Template>0</Template>"#);
    }

    #[test]
    fn test_application() {
        let b = XMLBuilder::new();
        let r = b.application("Lawgue").build();
        assert_eq!(
            str::from_utf8(&r).unwrap(),
            r#"<Application>Lawgue</Application>"#
        );
    }
}
