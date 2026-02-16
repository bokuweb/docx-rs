use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;
use std::io::Write;

use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Indent {
    pub start: Option<i32>,
    pub end: Option<i32>,
    pub special_indent: Option<SpecialIndentType>,
    pub start_chars: Option<i32>,
    // Internal, for reading
    pub hanging_chars: Option<i32>,
    pub first_line_chars: Option<i32>,
}

impl Indent {
    pub fn new(
        start: Option<i32>,
        special_indent: Option<SpecialIndentType>,
        end: Option<i32>,
        start_chars: Option<i32>,
    ) -> Indent {
        Indent {
            start,
            start_chars,
            end,
            special_indent,
            // Internal, for reading
            hanging_chars: None,
            first_line_chars: None,
        }
    }

    pub fn end(mut self, end: i32) -> Self {
        self.end = Some(end);
        self
    }

    pub fn hanging_chars(mut self, chars: i32) -> Self {
        self.hanging_chars = Some(chars);
        self
    }

    pub fn first_line_chars(mut self, chars: i32) -> Self {
        self.first_line_chars = Some(chars);
        self
    }
}

impl BuildXML for Indent {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .indent(
                self.start,
                self.special_indent,
                self.end.unwrap_or_default(),
                self.start_chars,
            )?
            .into_inner()
    }
}

impl Serialize for Indent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut t = serializer.serialize_struct("Indent", 3)?;
        t.serialize_field("start", &self.start)?;
        t.serialize_field("startChars", &self.start_chars)?;
        t.serialize_field("end", &self.end)?;
        t.serialize_field("specialIndent", &self.special_indent)?;
        t.serialize_field("hangingChars", &self.hanging_chars)?;
        t.serialize_field("firstLineChars", &self.first_line_chars)?;
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
    fn test_left() {
        let b = Indent::new(Some(20), None, None, None).build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:ind w:left="20" w:right="0" />"#
        );
    }

    #[test]
    fn test_first_line() {
        let b = Indent::new(Some(20), Some(SpecialIndentType::FirstLine(40)), None, None).build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:ind w:left="20" w:right="0" w:firstLine="40" />"#
        );
    }

    #[test]
    fn test_hanging() {
        let b = Indent::new(Some(20), Some(SpecialIndentType::Hanging(50)), None, None).build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:ind w:left="20" w:right="0" w:hanging="50" />"#
        );
    }
}
