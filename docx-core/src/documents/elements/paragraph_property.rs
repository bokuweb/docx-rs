use serde::Serialize;

use super::*;
use crate::documents::BuildXML;
use crate::types::{AlignmentType, SpecialIndentType};
use crate::xml_builder::*;

#[derive(Serialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ParagraphProperty {
    pub run_property: RunProperty,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<ParagraphStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numbering_property: Option<NumberingProperty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alignment: Option<Justification>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent: Option<Indent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_spacing: Option<LineSpacing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_next: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_lines: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_break_before: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub widow_control: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outline_lvl: Option<OutlineLvl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section_property: Option<SectionProperty>,
    pub tabs: Vec<Tab>,
    // read only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) div_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paragraph_property_change: Option<ParagraphPropertyChange>,
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

    pub fn numbering_property(mut self, np: NumberingProperty) -> Self {
        self.numbering_property = Some(np);
        self
    }

    pub fn line_spacing(mut self, spacing: LineSpacing) -> Self {
        self.line_spacing = Some(spacing);
        self
    }

    pub fn keep_next(mut self, v: bool) -> Self {
        self.keep_next = Some(v);
        self
    }

    pub fn keep_lines(mut self, v: bool) -> Self {
        self.keep_lines = Some(v);
        self
    }

    pub fn outline_lvl(mut self, v: usize) -> Self {
        if v >= 10 {
            return self;
        }
        self.outline_lvl = Some(OutlineLvl::new(v));
        self
    }

    pub fn page_break_before(mut self, v: bool) -> Self {
        self.page_break_before = Some(v);
        self
    }

    pub fn widow_control(mut self, v: bool) -> Self {
        self.widow_control = Some(v);
        self
    }

    pub fn add_tab(mut self, t: Tab) -> Self {
        self.tabs.push(t);
        self
    }

    pub fn section_property(mut self, s: SectionProperty) -> Self {
        self.section_property = Some(s);
        self
    }

    pub fn paragraph_property_change(mut self, p: ParagraphPropertyChange) -> Self {
        self.paragraph_property_change = Some(p);
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

fn inner_build(p: &ParagraphProperty) -> Vec<u8> {
    let mut b = XMLBuilder::new()
        .open_paragraph_property()
        .add_child(&p.run_property)
        .add_optional_child(&p.style)
        .add_optional_child(&p.numbering_property)
        .add_optional_child(&p.alignment)
        .add_optional_child(&p.indent)
        .add_optional_child(&p.line_spacing)
        .add_optional_child(&p.outline_lvl)
        .add_optional_child(&p.paragraph_property_change);

    if let Some(v) = p.keep_next {
        if v {
            b = b.keep_next()
        }
    }

    if let Some(v) = p.keep_lines {
        if v {
            b = b.keep_lines()
        }
    }

    if let Some(v) = p.page_break_before {
        if v {
            b = b.page_break_before()
        }
    }

    if let Some(v) = p.widow_control {
        if v {
            b = b.widow_control()
        }
    }

    if !p.tabs.is_empty() {
        b = b.open_tabs();
        for t in p.tabs.iter() {
            b = b.tab(t.val, t.leader, t.pos);
        }
        b = b.close();
    }

    b.close().build()
}

impl BuildXML for ParagraphProperty {
    fn build(&self) -> Vec<u8> {
        inner_build(self)
    }
}

impl BuildXML for Box<ParagraphProperty> {
    fn build(&self) -> Vec<u8> {
        inner_build(self)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::types::LineSpacingType;
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
    fn test_keep_next() {
        let c = ParagraphProperty::new();
        let b = c.keep_next(true).build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:pPr><w:rPr /><w:keepNext />
</w:pPr>"#
        );
    }

    #[test]
    fn test_outline_lvl() {
        let props = ParagraphProperty::new();
        let bytes = props.outline_lvl(1).build();
        assert_eq!(
            str::from_utf8(&bytes).unwrap(),
            r#"<w:pPr><w:rPr /><w:outlineLvl w:val="1" /></w:pPr>"#
        )
    }

    #[test]
    fn test_indent_json() {
        let c = ParagraphProperty::new();
        let b = c.indent(Some(20), Some(SpecialIndentType::FirstLine(10)), None, None);
        assert_eq!(
            serde_json::to_string(&b).unwrap(),
            r#"{"runProperty":{},"indent":{"start":20,"startChars":null,"end":null,"specialIndent":{"type":"firstLine","val":10},"hangingChars":null,"firstLineChars":null},"tabs":[]}"#
        );
    }

    #[test]
    fn test_line_spacing() {
        let props = ParagraphProperty::new();
        let spacing = LineSpacing::new()
            .line_rule(LineSpacingType::AtLeast)
            .line(100);
        let bytes = props.line_spacing(spacing).build();
        assert_eq!(
            str::from_utf8(&bytes).unwrap(),
            r#"<w:pPr><w:rPr /><w:spacing w:line="100" w:lineRule="atLeast" /></w:pPr>"#
        )
    }
}
