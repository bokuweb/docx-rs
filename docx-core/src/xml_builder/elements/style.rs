use super::{XMLBuilder, XmlEvent};
use crate::StyleType;

impl XMLBuilder {
    // Build w:style element
    // i.e. <w:style ... >
    pub(crate) fn open_style(mut self, style_type: StyleType, id: &str) -> Self {
        self.writer
            .write(
                XmlEvent::start_element("w:style")
                    .attr("w:type", &style_type.to_string())
                    .attr("w:styleId", id),
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
    fn test_declaration() {
        let b = XMLBuilder::new();
        let r = b
            .open_style(StyleType::Paragraph, "Heading")
            .close()
            .build();
        assert_eq!(
            str::from_utf8(&r).unwrap(),
            r#"<w:style w:type="paragraph" w:styleId="Heading" />"#
        );
    }
}
