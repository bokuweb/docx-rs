use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ParagraphPropertyChange(docx_rs::ParagraphPropertyChange);

#[wasm_bindgen(js_name = createParagraphPropertyChange)]
pub fn create_paragraph_property_change() -> ParagraphPropertyChange {
    ParagraphPropertyChange(docx_rs::ParagraphPropertyChange::new())
}

#[wasm_bindgen]
pub struct ParagraphProperty(docx_rs::ParagraphProperty);

#[wasm_bindgen(js_name = createParagraphProperty)]
pub fn create_paragraph_property() -> ParagraphProperty {
    ParagraphProperty(docx_rs::ParagraphProperty::new())
}

#[wasm_bindgen]
impl ParagraphProperty {
    pub fn align(mut self, alignment_type: docx_rs::AlignmentType) -> Self {
        self.0 = self.0.align(alignment_type);
        self
    }

    pub fn text_alignment(mut self, alignment_type: docx_rs::TextAlignmentType) -> Self {
        self.0 = self.0.text_alignment(alignment_type);
        self
    }

    pub fn adjust_right_ind(mut self, v: isize) -> Self {
        self.0 = self.0.adjust_right_ind(v);
        self
    }

    pub fn outline_lvl(mut self, level: usize) -> Self {
        self.0 = self.0.outline_lvl(level);
        self
    }

    pub fn style(mut self, style_id: &str) -> Self {
        self.0 = self.0.style(style_id);
        self
    }

    pub fn indent(
        mut self,
        left: i32,
        special_indent_kind: Option<docx_rs::SpecialIndentKind>,
        special_indent_size: Option<i32>,
        right: Option<i32>,
    ) -> Self {
        let special_indent = create_special_indent(special_indent_kind, special_indent_size);
        self.0 = self.0.indent(Some(left), special_indent, right, None);
        self
    }

    pub fn numbering(mut self, id: usize, level: usize) -> Self {
        let id = docx_rs::NumberingId::new(id);
        let level = docx_rs::IndentLevel::new(level);
        self.0 = self.0.numbering(id, level);
        self
    }

    pub fn line_spacing(mut self, spacing: LineSpacing) -> Self {
        self.0 = self.0.line_spacing(spacing.take());
        self
    }

    pub fn character_spacing(mut self, spacing: i32) -> Self {
        self.0 = self.0.character_spacing(spacing);
        self
    }

    pub fn keep_next(mut self, v: bool) -> Self {
        self.0 = self.0.keep_next(v);
        self
    }

    pub fn snap_to_grid(mut self, v: bool) -> Self {
        self.0 = self.0.snap_to_grid(v);
        self
    }

    pub fn keep_lines(mut self, v: bool) -> Self {
        self.0 = self.0.keep_lines(v);
        self
    }

    pub fn page_break_before(mut self, v: bool) -> Self {
        self.0 = self.0.page_break_before(v);
        self
    }

    pub fn widow_control(mut self, v: bool) -> Self {
        self.0 = self.0.widow_control(v);
        self
    }

    pub fn add_tab(
        mut self,
        val: Option<docx_rs::TabValueType>,
        leader: Option<docx_rs::TabLeaderType>,
        pos: Option<usize>,
    ) -> Self {
        self.0 = self.0.add_tab(docx_rs::Tab { val, leader, pos });
        self
    }

    pub fn paragraph_property_change(mut self, p: ParagraphPropertyChange) -> Self {
        self.0 = self.0.paragraph_property_change(p.take());
        self
    }

    pub fn frame_property(mut self, p: FrameProperty) -> Self {
        self.0 = self.0.frame_property(p.take());
        self
    }

    pub fn run_property(mut self, p: RunProperty) -> Self {
        self.0 = self.0.run_property(p.take());
        self
    }
}

impl ParagraphProperty {
    pub fn take(self) -> docx_rs::ParagraphProperty {
        self.0
    }
}

#[wasm_bindgen]
impl ParagraphPropertyChange {
    pub fn author(mut self, author: &str) -> Self {
        self.0 = self.0.author(author);
        self
    }

    pub fn date(mut self, date: &str) -> Self {
        self.0 = self.0.date(date);
        self
    }

    // TODO: For now only numbering supported.
    pub fn numbering(mut self, id: usize, level: usize) -> Self {
        let id = docx_rs::NumberingId::new(id);
        let level = docx_rs::IndentLevel::new(level);
        self.0.property = Box::new(self.0.property.numbering(id, level));
        self
    }

    pub fn align(mut self, alignment_type: docx_rs::AlignmentType) -> Self {
        self.0.property = Box::new(self.0.property.align(alignment_type));
        self
    }

    pub fn style(mut self, style_id: &str) -> Self {
        self.0.property = Box::new(self.0.property.style(style_id));
        self
    }

    pub fn indent(
        mut self,
        left: i32,
        special_indent_kind: Option<docx_rs::SpecialIndentKind>,
        special_indent_size: Option<i32>,
    ) -> Self {
        let special_indent = create_special_indent(special_indent_kind, special_indent_size);
        self.0.property = Box::new(
            self.0
                .property
                .indent(Some(left), special_indent, None, None),
        );
        self
    }
}

impl ParagraphPropertyChange {
    pub fn take(self) -> docx_rs::ParagraphPropertyChange {
        self.0
    }
}
