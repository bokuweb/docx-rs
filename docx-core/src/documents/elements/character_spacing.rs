use crate::documents::BuildXML;
use crate::xml_builder::*;

use serde::*;

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[cfg_attr(feature = "wasm", derive(ts_rs::TS))]
#[cfg_attr(feature = "wasm", ts(export))]
pub struct CharacterSpacing(pub i32);

impl CharacterSpacing {
    pub fn new(s: i32) -> CharacterSpacing {
        Self(s)
    }
}

impl BuildXML for CharacterSpacing {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.spacing(self.0).build()
    }
}

impl Serialize for CharacterSpacing {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.0)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_spacing() {
        let b = CharacterSpacing::new(200).build();
        assert_eq!(str::from_utf8(&b).unwrap(), r#"<w:spacing w:val="200" />"#);
    }

    #[test]
    fn test_spacing_json() {
        let s = CharacterSpacing(100);
        assert_eq!(serde_json::to_string(&s).unwrap(), r#"100"#);
    }
}
