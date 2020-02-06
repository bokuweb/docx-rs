use serde::{Deserialize, Serialize, Serializer};

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Underline {
    val: String,
}

impl Underline {
    pub fn new(val: impl Into<String>) -> Underline {
        Underline { val: val.into() }
    }
}

impl BuildXML for Underline {
    fn build(&self) -> Vec<u8> {
        XMLBuilder::new().underline(&self.val).build()
    }
}

impl Serialize for Underline {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.val)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_underline() {
        let c = Underline::new("single");
        let b = c.build();
        assert_eq!(str::from_utf8(&b).unwrap(), r#"<w:u w:val="single" />"#);
    }
}
