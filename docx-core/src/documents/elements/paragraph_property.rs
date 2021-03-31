use serde::Serialize;

use super::*;
use crate::documents::BuildXML;
use crate::types::{AlignmentType, SpecialIndentType};
use crate::xml_builder::*;

#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParagraphProperty {
    pub run_property: RunProperty,
    pub style: Option<ParagraphStyle>,
    pub numbering_property: Option<NumberingProperty>,
    pub alignment: Option<Justification>,
    pub indent: Option<Indent>,
    pub line_height: Option<u32>,
}

impl Default for ParagraphProperty {
    fn default() -> Self {
        ParagraphProperty {
            run_property: RunProperty::new(),
            style: None,
            numbering_property: None,
            alignment: None,
            indent: None,
            line_height: None,
        }
    }
}

// 17.3.1.26
// pPr (Paragraph Properties)
// This element specifies a set of paragraph properties which shall be applied to the contents of the parent
// paragraph after all style/numbering/table properties have been applied to the text. These properties are defined
// as direct formatting, since they are directly applied to the paragraph and supersede any formatting from styles.
impl ParagraphProperty {
    pub fn new() -> ParagraphProperty {
        Default::default()
    }

    pub fn align(mut self, alignment_type: AlignmentType) -> Self {
        self.alignment = Some(Justification::new(alignment_type.to_string()));
        self
    }

    pub fn style(mut self, style_id: &str) -> Self {
        self.style = Some(ParagraphStyle::new(Some(style_id)));
        self
    }

    pub fn indent(
        mut self,
        left: Option<i32>,
        special_indent: Option<SpecialIndentType>,
        end: Option<i32>,
        start_chars: Option<i32>,
    ) -> Self {
        self.indent = Some(Indent::new(left, special_indent, end, start_chars));
        self
    }

    pub fn numbering(mut self, id: NumberingId, level: IndentLevel) -> Self {
        self.numbering_property = Some(NumberingProperty::new().add_num(id, level));
        self
    }

    pub fn line_height(mut self, h: u32) -> Self {
        self.line_height = Some(h);
        self
    }

    pub(crate) fn hanging_chars(mut self, chars: i32) -> Self {
        if let Some(indent) = self.indent {
            self.indent = Some(indent.hanging_chars(chars));
        }
        self
    }

    pub(crate) fn first_line_chars(mut self, chars: i32) -> Self {
        if let Some(indent) = self.indent {
            self.indent = Some(indent.first_line_chars(chars));
        }
        self
    }
}

impl BuildXML for ParagraphProperty {
    fn build(&self) -> Vec<u8> {
        let spacing = if let Some(s) = self.line_height {
            Some(Spacing::new(crate::SpacingType::Line(s)))
        } else {
            None
        };
        XMLBuilder::new()
            .open_paragraph_property()
            .add_child(&self.run_property)
            .add_optional_child(&self.style)
            .add_optional_child(&self.numbering_property)
            .add_optional_child(&self.alignment)
            .add_optional_child(&self.indent)
            .add_optional_child(&spacing)
            .close()
            .build()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_default() {
        let c = ParagraphProperty::new();
        let b = c.build();
        assert_eq!(str::from_utf8(&b).unwrap(), r#"<w:pPr><w:rPr /></w:pPr>"#);
    }

    #[test]
    fn test_alignment() {
        let c = ParagraphProperty::new();
        let b = c.align(AlignmentType::Right).build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:pPr><w:rPr /><w:jc w:val="right" /></w:pPr>"#
        );
    }

    #[test]
    fn test_indent() {
        let c = ParagraphProperty::new();
        let b = c.indent(Some(20), None, None, None).build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:pPr><w:rPr /><w:ind w:left="20" w:right="0" /></w:pPr>"#
        );
    }

    #[test]
    fn test_indent_json() {
        let c = ParagraphProperty::new();
        let b = c.indent(Some(20), Some(SpecialIndentType::FirstLine(10)), None, None);
        assert_eq!(
            serde_json::to_string(&b).unwrap(),
            r#"{"runProperty":{"sz":null,"szCs":null,"color":null,"highlight":null,"underline":null,"bold":null,"boldCs":null,"italic":null,"italicCs":null,"vanish":null,"spacing":null,"fonts":null,"textBorder":null},"style":null,"numberingProperty":null,"alignment":null,"indent":{"start":20,"startChars":null,"end":null,"specialIndent":{"type":"firstLine","val":10}},"lineHeight":null}"#
        );
    }
}
