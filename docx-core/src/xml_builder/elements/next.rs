use super::XMLBuilder;
use super::XmlEvent;

impl XMLBuilder {
    // i.e. <w:next ... >
    pub(crate) fn next(mut self, val: &str) -> Self {
        self.writer
            .write(XmlEvent::start_element("w:next").attr("w:val", val))
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
    fn test_next() {
        let b = XMLBuilder::new();
        let r = b.next("Normal").build();
        assert_eq!(str::from_utf8(&r).unwrap(), r#"<w:next w:val="Normal" />"#);
    }
}
