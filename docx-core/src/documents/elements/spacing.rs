use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;

use serde::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Spacing {
    spacing_type: SpacingType,
}

impl Spacing {
    pub fn new(s: SpacingType) -> Spacing {
        Self { spacing_type: s }
    }
}

impl BuildXML for Spacing {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.spacing(self.spacing_type).build()
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
        let b = Spacing::new(SpacingType::Value(200)).build();
        assert_eq!(str::from_utf8(&b).unwrap(), r#"<w:spacing w:val="200" />"#);
    }

    #[test]
    fn test_spacing_json() {
        let s = Spacing {
            spacing_type: SpacingType::Value(100),
        };
        assert_eq!(
            serde_json::to_string(&s).unwrap(),
            r#"{"spacingType":{"type":"value","data":100}}"#
        );
    }
}
