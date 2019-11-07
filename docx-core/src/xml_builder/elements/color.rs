use super::XMLBuilder;
use super::XmlEvent;

impl XMLBuilder {
    // i.e. <w:color ... >
    pub(crate) fn color(mut self, val: &str) -> Self {
        self.writer
            .write(XmlEvent::start_element("w:color").attr("w:val", val))
            .expect("should write to buf");
        self.close()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_color() {
        let b = XMLBuilder::new();
        let r = b.color("2E74B5").build();
        assert_eq!(str::from_utf8(&r).unwrap(), r#"<w:color w:val="2E74B5" />"#);
    }
}
