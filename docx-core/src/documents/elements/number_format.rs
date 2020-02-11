use serde::{Serialize, Serializer};

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, PartialEq)]
pub struct NumberFormat {
    val: String,
}

impl NumberFormat {
    pub fn new(val: impl Into<String>) -> Self {
        Self { val: val.into() }
    }
}

impl BuildXML for NumberFormat {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.number_format(&self.val).build()
    }
}

impl Serialize for NumberFormat {
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
    fn test_start() {
        let c = NumberFormat::new("decimal");
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:numFmt w:val="decimal" />"#
        );
    }
}
