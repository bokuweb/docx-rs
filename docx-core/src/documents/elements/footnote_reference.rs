use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::io::Write;

use crate::documents::BuildXML;
use crate::{xml_builder::*, Footnote, Paragraph};

#[derive(Debug, Clone, PartialEq)]

pub struct FootnoteReference {
    pub id: usize,
    pub custom_mark_follows: Option<bool>,
    pub style: String,
    pub content: Vec<Paragraph>,
}

impl FootnoteReference {
    pub fn new(id: usize) -> Self {
        FootnoteReference {
            id,
            custom_mark_follows: None,
            style: "FootnoteReference".to_string(),
            content: vec![],
        }
    }
    /// Suppress the automatic reference marker when a custom mark follows.
    pub fn custom_mark_follows(mut self, value: bool) -> Self {
        self.custom_mark_follows = Some(value);
        self
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
            custom_mark_follows: None,
            style: "FootnoteReference".to_string(),
            content: footnote.content,
        }
    }
}

impl BuildXML for FootnoteReference {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .footnote_reference(self.id, self.custom_mark_follows)?
            .into_inner()
    }
}

impl Serialize for FootnoteReference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut t = serializer.serialize_struct(
            "FootnoteReference",
            1 + usize::from(self.custom_mark_follows.is_some()),
        )?;
        t.serialize_field("id", &self.id)?;
        if let Some(value) = self.custom_mark_follows {
            t.serialize_field("customMarkFollows", &value)?;
        }
        t.end()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
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

    #[test]
    fn test_custom_mark_roundtrip() {
        use crate::{reader::ElementReader, Run, RunChild};
        for value in [true, false] {
            let reference = FootnoteReference::new(29).custom_mark_follows(value);
            let xml = String::from_utf8(reference.build()).unwrap();
            assert_eq!(
                xml,
                format!(
                    r#"<w:footnoteReference w:id="29" w:customMarkFollows="{}" />"#,
                    usize::from(value)
                )
            );
            let run_xml = format!(
                r#"<w:r xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">{xml}<w:t>**</w:t></w:r>"#
            );
            let mut reader = crate::reader::EventReader::new(run_xml.as_bytes());
            let run = Run::read(&mut reader, &[]).unwrap();
            assert_eq!(
                run.children[0],
                RunChild::FootnoteReference(reference.clone())
            );
            assert_eq!(
                serde_json::to_value(reference).unwrap(),
                serde_json::json!({"id":29,"customMarkFollows":value})
            );
        }
    }
}
