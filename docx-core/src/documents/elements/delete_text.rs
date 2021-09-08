use serde::{Deserialize, Serialize};

use crate::documents::BuildXML;
use crate::escape::escape;
use crate::xml_builder::*;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DeleteText {
    text: String,
    preserve_space: bool,
}

impl DeleteText {
    pub fn new(text: impl Into<String>) -> DeleteText {
        DeleteText {
            text: escape(&text.into()),
            preserve_space: true,
        }
    }
}

impl BuildXML for DeleteText {
    fn build(&self) -> Vec<u8> {
        XMLBuilder::new().delete_text(&self.text, true).build()
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
        let b = DeleteText::new("Hello").build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:delText xml:space="preserve">Hello</w:delText>"#
        );
    }

    #[test]
    fn test_escape() {
        let b = DeleteText::new("<div />").build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:delText xml:space="preserve">&lt;div /&gt;</w:delText>"#
        );
    }
}
