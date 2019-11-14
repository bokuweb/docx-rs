use super::XMLBuilder;
use super::XmlEvent;

impl XMLBuilder {
    pub(crate) fn open_settings(mut self) -> Self {
        self.writer
            .write(XmlEvent::start_element("w:settings").attr(
                "xmlns:w",
                "http://schemas.openxmlformats.org/wordprocessingml/2006/main",
            ))
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
        let r = b.open_settings().close().build();
        assert_eq!(
            str::from_utf8(&r).unwrap(),
            r#"<w:settings xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" />"#
        );
    }
}
