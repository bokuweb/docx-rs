use serde::{Deserialize, Serialize, Serializer};

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[cfg_attr(feature = "wasm", derive(ts_rs::TS))]
#[cfg_attr(feature = "wasm", ts(export))]
pub struct Highlight(pub String);

impl Highlight {
    pub fn new(val: impl Into<String>) -> Highlight {
        Highlight(val.into())
    }
}

impl BuildXML for Highlight {
    fn build(&self) -> Vec<u8> {
        XMLBuilder::new().highlight(&self.0).build()
    }
}

impl Serialize for Highlight {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_highlight() {
        let c = Highlight::new("FFFFFF");
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:highlight w:val="FFFFFF" />"#
        );
    }

    #[test]
    fn test_highlight_json() {
        let c = Highlight::new("FFFFFF");

        assert_eq!(serde_json::to_string(&c).unwrap(), r#""FFFFFF""#);
    }
}
