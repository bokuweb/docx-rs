use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::documents::BuildXML;
use crate::{xml_builder::*, Footnote, Paragraph};

#[derive(Debug, Clone, PartialEq)]

pub struct FootnoteReference {
    pub id: usize,
    pub style: String,
    pub content: Vec<Paragraph>,
}

impl FootnoteReference {
    pub fn new(id: usize) -> Self {
        FootnoteReference {
            id,
            style: "FootnoteReference".to_string(),
            content: vec![],
        }
    }
    /// Add footnote content as a Paragraph
    pub fn footnote(&mut self, p: Paragraph) {
        self.content.push(p)
    }
}
impl From<Footnote> for FootnoteReference {
    fn from(footnote: Footnote) -> Self {
        FootnoteReference {
            id: footnote.id,
            style: "FootnoteReference".to_string(),
            content: footnote.content,
        }
    }
}

impl BuildXML for FootnoteReference {
    fn build(&self) -> Vec<u8> {
        XMLBuilder::new().footnote_reference(self.id).build()
    }
}

impl Serialize for FootnoteReference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut t = serializer.serialize_struct("FootnoteReference", 2)?;
        t.serialize_field("id", &self.id)?;
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
    fn test_footnotereference_build() {
        let b = FootnoteReference::new(1).build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:footnoteReference w:id="1" />"#
        );
    }

    #[test]
    fn test_footnotereference_json() {
        let t = FootnoteReference::new(1);
        assert_eq!(serde_json::to_string(&t).unwrap(), r#"{"id":1}"#);
    }
}
