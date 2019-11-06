use std::fmt;

use super::XMLBuilder;
use super::XmlEvent;

impl XMLBuilder {
    // i.e. <w:name ... >
    only_str_val_el!(name, "w:name");
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
        let r = b.name("Heading").build();
        assert_eq!(str::from_utf8(&r).unwrap(), r#"<w:name w:val="Heading" />"#);
    }
}
