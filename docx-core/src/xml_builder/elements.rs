use super::XMLBuilder;
use super::XmlEvent;
use crate::types::*;

impl XMLBuilder {
    // i.e. <w:body... >
    opened_el!(open_body, "w:body");
    // i.e. <w:basedOn ... >
    only_str_val_el!(based_on, "w:basedOn");
    // i.e. <w:t ... >
    // i.e. <w:sz ... >
    pub(crate) fn text(mut self, text: &str, preserve_space: bool) -> Self {
        let space = if preserve_space {
            "preserve"
        } else {
            "default"
        };
        self.writer
            .write(XmlEvent::start_element("w:t").attr("xml:space", space))
            .expect("should write to buf");
        self.writer.write(text).expect("should write to buf");
        self.close()
    }
    // i.e. <w:r ... >
    opened_el!(open_run, "w:r");
    opened_el!(open_run_property, "w:rPr");
    opened_el!(open_run_property_default, "w:rPrDefault");
    // i.e. <w:qFormat ... >
    closed_el!(q_format, "w:qFormat");
    // i.e. <w:p ... >
    opened_el!(open_paragraph, "w:p");
    opened_el!(open_paragraph_property, "w:pPr");
    opened_el!(open_doc_defaults, "w:docDefaults");
    // i.e. <w:name ... >
    only_str_val_el!(name, "w:name");
    // i.e. <w:sz ... >
    pub(crate) fn sz(mut self, val: usize) -> Self {
        self.writer
            .write(XmlEvent::start_element("w:sz").attr("w:val", &format!("{}", val)))
            .expect("should write to buf");
        self.close()
    }
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
    // i.e. <w:next ... >
    pub(crate) fn next(mut self, val: &str) -> Self {
        self.writer
            .write(XmlEvent::start_element("w:next").attr("w:val", val))
            .expect("should write to buf");
        self.close()
    }

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
    fn test_sz() {
        let b = XMLBuilder::new();
        let r = b.sz(20).build();
        assert_eq!(str::from_utf8(&r).unwrap(), r#"<w:sz w:val="20" />"#);
    }

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

    #[test]
    fn test_next() {
        let b = XMLBuilder::new();
        let r = b.next("Normal").build();
        assert_eq!(str::from_utf8(&r).unwrap(), r#"<w:next w:val="Normal" />"#);
    }

    #[test]
    fn test_name() {
        let b = XMLBuilder::new();
        let r = b.name("Heading").build();
        assert_eq!(str::from_utf8(&r).unwrap(), r#"<w:name w:val="Heading" />"#);
    }

    #[test]
    fn test_color() {
        let b = XMLBuilder::new();
        let r = b.color("2E74B5").build();
        assert_eq!(str::from_utf8(&r).unwrap(), r#"<w:color w:val="2E74B5" />"#);
    }

    #[test]
    fn test_based_on() {
        let b = XMLBuilder::new();
        let r = b.based_on("Normal").build();
        assert_eq!(
            str::from_utf8(&r).unwrap(),
            r#"<w:basedOn w:val="Normal" />"#
        );
    }
}
