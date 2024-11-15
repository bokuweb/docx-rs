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
    // Child elements: Sequence [1..1]
    //1. w:pStyle
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<ParagraphStyle>,
    //2. w:keepNext
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_next: Option<bool>,
    //3. w:keepLines
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_lines: Option<bool>,
    //4. w:pageBreakBefore
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_break_before: Option<bool>,
    //5. w:framePr
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_property: Option<FrameProperty>,
    //6. w:widowControl
    #[serde(skip_serializing_if = "Option::is_none")]
    pub widow_control: Option<bool>,
    //7. w:numPr
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numbering_property: Option<NumberingProperty>,
    //8. w:suppressLineNumbers
    //9. w:pBdr
    #[serde(skip_serializing_if = "Option::is_none")]
    pub borders: Option<ParagraphBorders>,
    //10. w:shd
    //11. w:tabs
    pub tabs: Vec<Tab>,
    //12. w:suppressAutoHyphens
    //13. w:kinsoku
    //14. w:wordWrap
    //15. w:overflowPunct
    //16. w:topLinePunct
    //17. w:autoSpaceDE
    //18. w:autoSpaceDN
    //19. w:bidi
    //20. w:adjustRightInd
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjust_right_ind: Option<AdjustRightInd>,
    //21. w:snapToGrid
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snap_to_grid: Option<bool>,
    //22. w:spacing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_spacing: Option<LineSpacing>,
    //23. w:ind
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent: Option<Indent>,
    //24. w:contextualSpacing
    //25. w:mirrorIndents
    //26. w:suppressOverlap
    //27. w:jc
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alignment: Option<Justification>,
    //28. w:textDirection
    //29. w:textAlignment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_alignment: Option<TextAlignment>,
    //30. w:textboxTightWrap
    //31. w:outlineLvl
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outline_lvl: Option<OutlineLvl>,
    //32. w:divId
    // read only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) div_id: Option<String>,
    //33. w:cnfStyle
    //34. w:rPr
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_property: Option<RunProperty>,
    //35. w:sectPr
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section_property: Option<SectionProperty>,
    //36. w:pPrChange
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

    pub fn character_spacing(mut self, spacing: i32) -> Self {
        let run_property = self.run_property.get_or_insert_with(RunProperty::new);
        run_property.character_spacing = Some(CharacterSpacing::new(spacing));
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
            .add_optional_child(&self.style)?
            .apply_if(self.keep_next, |b| b.keep_next())?
            .apply_if(self.keep_lines, |b| b.keep_lines())?
            .apply_if(self.page_break_before, |b| b.page_break_before())?
            .add_optional_child(&self.frame_property)?
            .apply_opt(self.widow_control, |flag, b| {
                b.widow_control(if flag { "1" } else { "0" })
            })?
            .add_optional_child(&self.numbering_property)?
            .add_optional_child(&self.borders)?
            .apply_if(!self.tabs.is_empty(), |b| {
                b.open_tabs()?
                    .apply_each(&self.tabs, |tab, b| b.tab(tab.val, tab.leader, tab.pos))?
                    .close()
            })?
            .add_optional_child(&self.adjust_right_ind)?
            .apply_opt(self.snap_to_grid, |v, b| b.snap_to_grid(v))?
            .add_optional_child(&self.line_spacing)?
            .add_optional_child(&self.indent)?
            .add_optional_child(&self.alignment)?
            .add_optional_child(&self.text_alignment)?
            .add_optional_child(&self.outline_lvl)?
            .add_optional_child(&self.run_property)?
            .add_optional_child(&self.section_property)?
            .add_optional_child(&self.paragraph_property_change)?
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
        assert_eq!(str::from_utf8(&b).unwrap(), r#"<w:pPr />"#);
    }

    #[test]
    fn test_alignment() {
        let c = ParagraphProperty::new();
        let b = c.align(AlignmentType::Right).build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:pPr><w:jc w:val="right" /></w:pPr>"#
        );
    }

    #[test]
    fn test_indent() {
        let c = ParagraphProperty::new();
        let b = c.indent(Some(20), None, None, None).build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:pPr><w:ind w:left="20" w:right="0" /></w:pPr>"#
        );
    }

    #[test]
    fn test_keep_next() {
        let c = ParagraphProperty::new();
        let b = c.keep_next(true).build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:pPr><w:keepNext /></w:pPr>"#
        );
    }

    #[test]
    fn test_outline_lvl() {
        let props = ParagraphProperty::new();
        let bytes = props.outline_lvl(1).build();
        assert_eq!(
            str::from_utf8(&bytes).unwrap(),
            r#"<w:pPr><w:outlineLvl w:val="1" /></w:pPr>"#
        )
    }

    #[test]
    fn test_indent_json() {
        let c = ParagraphProperty::new();
        let b = c.indent(Some(20), Some(SpecialIndentType::FirstLine(10)), None, None);
        assert_eq!(
            serde_json::to_string(&b).unwrap(),
            r#"{"tabs":[],"indent":{"start":20,"startChars":null,"end":null,"specialIndent":{"type":"firstLine","val":10},"hangingChars":null,"firstLineChars":null}}"#
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
            r#"<w:pPr><w:spacing w:line="100" w:lineRule="atLeast" /></w:pPr>"#
        )
    }
}
