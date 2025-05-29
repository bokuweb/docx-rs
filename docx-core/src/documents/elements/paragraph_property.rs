use serde::Serialize;
use std::io::Write;

use super::*;
use crate::documents::BuildXML;
use crate::types::{AlignmentType, SpecialIndentType};
use crate::ParagraphBorderPosition;
use crate::{xml_builder::*, TextAlignmentType};

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
    pub bidi: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_break_before: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub widow_control: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outline_lvl: Option<OutlineLvl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section_property: Option<SectionProperty>,
    pub tabs: Vec<Tab>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paragraph_property_change: Option<ParagraphPropertyChange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub borders: Option<ParagraphBorders>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_property: Option<FrameProperty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_alignment: Option<TextAlignment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjust_right_ind: Option<AdjustRightInd>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snap_to_grid: Option<bool>,
    // read only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) div_id: Option<String>,
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

    pub fn character_spacing(mut self, spacing: i32) -> Self {
        self.run_property.character_spacing = Some(CharacterSpacing::new(spacing));
        self
    }

    pub fn snap_to_grid(mut self, v: bool) -> Self {
        self.snap_to_grid = Some(v);
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
            // clamped
            self.outline_lvl = Some(OutlineLvl::new(9));
            return self;
        }
        self.outline_lvl = Some(OutlineLvl::new(v));
        self
    }

    pub fn page_break_before(mut self, v: bool) -> Self {
        self.page_break_before = Some(v);
        self
    }
    pub fn bidi(mut self, v: bool) -> Self {
        self.bidi = Some(v);
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

    pub fn frame_property(mut self, s: FrameProperty) -> Self {
        self.frame_property = Some(s);
        self
    }

    pub fn run_property(mut self, s: RunProperty) -> Self {
        self.run_property = s;
        self
    }

    pub fn text_alignment(mut self, s: TextAlignmentType) -> Self {
        self.text_alignment = Some(TextAlignment::new(s));
        self
    }

    pub fn adjust_right_ind(mut self, s: isize) -> Self {
        self.adjust_right_ind = Some(AdjustRightInd::new(s));
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

    pub fn set_borders(mut self, borders: ParagraphBorders) -> Self {
        self.borders = Some(borders);
        self
    }

    pub fn set_border(mut self, border: ParagraphBorder) -> Self {
        self.borders = Some(self.borders.unwrap_or_default().set(border));
        self
    }

    pub fn clear_border(mut self, position: ParagraphBorderPosition) -> Self {
        self.borders = Some(self.borders.unwrap_or_default().clear(position));
        self
    }

    pub fn clear_all_borders(mut self) -> Self {
        self.borders = Some(self.borders.unwrap_or_default().clear_all());
        self
    }
}

impl BuildXML for ParagraphProperty {
    fn build_to<W: Write>(
        &self,
        stream: xml::writer::EventWriter<W>,
    ) -> xml::writer::Result<xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_paragraph_property()?
            .add_child(&self.run_property)?
            .add_optional_child(&self.section_property)?
            .add_optional_child(&self.style)?
            .add_optional_child(&self.numbering_property)?
            .add_optional_child(&self.frame_property)?
            .add_optional_child(&self.alignment)?
            .add_optional_child(&self.indent)?
            .add_optional_child(&self.line_spacing)?
            .add_optional_child(&self.outline_lvl)?
            .add_optional_child(&self.paragraph_property_change)?
            .add_optional_child(&self.borders)?
            .add_optional_child(&self.text_alignment)?
            .add_optional_child(&self.adjust_right_ind)?
            .apply_opt(self.snap_to_grid, |v, b| b.snap_to_grid(v))?
            .apply_if(self.keep_next, |b| b.keep_next())?
            .apply_if(self.keep_lines, |b| b.keep_lines())?
            .apply_if(self.page_break_before, |b| b.page_break_before())?
            .apply_if(self.bidi, |b| b.bidi())?
            .apply_opt(self.widow_control, |flag, b| {
                b.widow_control(if flag { "1" } else { "0" })
            })?
            .apply_if(!self.tabs.is_empty(), |b| {
                b.open_tabs()?
                    .apply_each(&self.tabs, |tab, b| b.tab(tab.val, tab.leader, tab.pos))?
                    .close()
            })?
            .close()?
            .into_inner()
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
    fn test_bidi() {
        let c = ParagraphProperty::new().bidi(true);
        let b = c.build();
        println!("-----Test bidi: {}", str::from_utf8(&b).unwrap());
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:pPr><w:rPr /><w:bidi /></w:pPr>"#
        );
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
            r#"<w:pPr><w:rPr /><w:keepNext /></w:pPr>"#
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
