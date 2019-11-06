use super::XMLBuilder;
use super::XmlEvent;

impl XMLBuilder {
    // i.e. <w:qFormat ... >
    closed_el!(q_format, "w:qFormat");
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_q_format() {
        let b = XMLBuilder::new();
        let r = b.q_format().build();
        assert_eq!(str::from_utf8(&r).unwrap(), r#"<w:qFormat />"#);
    }
}
