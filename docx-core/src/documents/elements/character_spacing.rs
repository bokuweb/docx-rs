use crate::documents::BuildXML;
use crate::xml_builder::*;

use serde::*;

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CharacterSpacing {
    value: i32,
}

impl CharacterSpacing {
    pub fn new(s: i32) -> CharacterSpacing {
        Self { value: s }
    }
}

impl BuildXML for CharacterSpacing {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.spacing(self.value).build()
    }
}

impl Serialize for CharacterSpacing {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.value)
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
        let s = CharacterSpacing { value: 100 };
        assert_eq!(serde_json::to_string(&s).unwrap(), r#"100"#);
    }
}
