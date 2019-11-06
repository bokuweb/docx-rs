use super::XMLBuilder;
use super::XmlEvent;

impl XMLBuilder {
    // i.e. <w:sz ... >
    pub(crate) fn sz(mut self, val: usize) -> Self {
        self.writer
            .write(XmlEvent::start_element("w:sz").attr("w:val", &format!("{}", val)))
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
    fn test_name() {
        let b = XMLBuilder::new();
        let r = b.sz(20).build();
        assert_eq!(str::from_utf8(&r).unwrap(), r#"<w:sz w:val="20" />"#);
    }
}
