use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug)]
pub struct Name {
    name: String,
}

impl Name {
    pub fn new(name: impl Into<String>) -> Name {
        Name { name: name.into() }
    }
}

impl BuildXML for Name {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.name(&self.name).build()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_build() {
        let c = Name::new("Heading");
        let b = c.build();
        assert_eq!(str::from_utf8(&b).unwrap(), r#"<w:name w:val="Heading" />"#);
    }
}
