use super::XMLBuilder;
use super::XmlEvent;

impl XMLBuilder {
    // Build RelationShips element
    // i.e. <Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
    open!(open_relationships, "Relationships", "xmlns");

    // Build Relationship
    closed!(relationship, "Relationship", "Id", "Type", "Target");
}

#[cfg(test)]
mod tests {

    use super::XMLBuilder;
    use std::str;

    #[test]
    fn test_open_relationships() {
        let b = XMLBuilder::new();
        let r = b
            .open_relationships("http://example")
            .plain_text("child")
            .close()
            .build();
        assert_eq!(
            str::from_utf8(&r).unwrap(),
            r#"<Relationships xmlns="http://example">child</Relationships>"#
        );
    }

    #[test]
    fn test_relationship() {
        let b = XMLBuilder::new();
        let r = b.relationship("rId1", "http://example", "core.xml").build();
        assert_eq!(
            str::from_utf8(&r).unwrap(),
            r#"<Relationship Id="rId1" Type="http://example" Target="core.xml" />"#
        );
    }
}
