use serde::{Deserialize, Serialize, Serializer};

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Color {
    val: String,
}

impl Color {
    pub fn new(val: impl Into<String>) -> Color {
        Color { val: val.into() }
    }
}

impl BuildXML for Color {
    fn build(&self) -> Vec<u8> {
        XMLBuilder::new().color(&self.val).build()
    }
}

impl Serialize for Color {
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
    fn test_build() {
        let c = Color::new("FFFFFF");
        let b = c.build();
        assert_eq!(str::from_utf8(&b).unwrap(), r#"<w:color w:val="FFFFFF" />"#);
    }
}
