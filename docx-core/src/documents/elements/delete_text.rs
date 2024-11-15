use serde::{Deserialize, Serialize};
use std::io::Write;

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

    pub(crate) fn without_escape(text: impl Into<String>) -> DeleteText {
        DeleteText {
            text: text.into(),
            preserve_space: true,
        }
    }
}

impl BuildXML for DeleteText {
    fn build_to<W: Write>(
        &self,
        stream: xml::writer::EventWriter<W>,
    ) -> xml::writer::Result<xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .delete_text(&self.text, true)?
            .into_inner()
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
