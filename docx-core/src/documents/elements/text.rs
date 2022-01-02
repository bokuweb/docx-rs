use serde::ser::{Serialize, SerializeStruct, Serializer};
use serde::Deserialize;

use crate::documents::BuildXML;
use crate::escape::escape;
use crate::xml_builder::*;

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Text {
    pub text: String,
    pub preserve_space: bool,
}

impl Text {
    pub fn new(text: impl Into<String>) -> Text {
        Text {
            text: escape(&text.into()),
            preserve_space: true,
        }
    }
}

impl BuildXML for Text {
    fn build(&self) -> Vec<u8> {
        XMLBuilder::new().text(&self.text, true).build()
    }
}

impl Serialize for Text {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut t = serializer.serialize_struct("Text", 2)?;
        t.serialize_field("preserveSpace", &self.preserve_space)?;
        t.serialize_field("text", &self.text)?;
        t.end()
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
        let b = Text::new("Hello").build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:t xml:space="preserve">Hello</w:t>"#
        );
    }

    #[test]
    fn test_json() {
        let t = Text::new("Hello");
        assert_eq!(
            serde_json::to_string(&t).unwrap(),
            r#"{"preserveSpace":true,"text":"Hello"}"#
        );
    }
}
