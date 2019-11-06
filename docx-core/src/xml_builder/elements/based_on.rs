use super::XMLBuilder;
use super::XmlEvent;

impl XMLBuilder {
    // i.e. <w:basedOn ... >
    only_str_val_el!(based_on, "w:basedOn");
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

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
