use serde::{Serialize, Serializer};

use std::str::FromStr;

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Name {
    name: String,
}

impl Name {
    pub fn new(name: impl Into<String>) -> Name {
        Name { name: name.into() }
    }

    pub fn starts_with(&self, s: &str) -> bool {
        self.name.starts_with(s)
    }

    pub fn is_heading(&self) -> bool {
        self.name.to_lowercase().starts_with("heading")
    }

    pub fn get_heading_number(&self) -> Option<usize> {
        let replaced = self.name.to_lowercase().replace("heading ", "");
        if let Ok(n) = usize::from_str(&replaced) {
            Some(n)
        } else {
            None
        }
    }
}

impl BuildXML for Name {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.name(&self.name).build()
    }
}

impl Serialize for Name {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.name)
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
