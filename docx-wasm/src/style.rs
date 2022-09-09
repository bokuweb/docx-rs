use super::*;
use docx_rs::{BorderType, TextBorder, VertAlignType, WidthType};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Style(docx_rs::Style);

#[wasm_bindgen(js_name = createStyle)]
pub fn create_style(style_id: &str, style_type: docx_rs::StyleType) -> Style {
    Style(docx_rs::Style::new(style_id, style_type))
}

#[wasm_bindgen]
impl Style {
    pub fn name(mut self, name: &str) -> Self {
        self.0.name = docx_rs::Name::new(name);
        self
    }

    pub fn based_on(mut self, base: &str) -> Self {
        self.0.based_on = Some(docx_rs::BasedOn::new(base));
        self
    }

    pub fn size(mut self, size: usize) -> Self {
        self.0.run_property = self.0.run_property.size(size);
        self
    }

    pub fn color(mut self, color: &str) -> Self {
        self.0.run_property = self.0.run_property.color(color);
        self
    }

    pub fn highlight(mut self, color: &str) -> Self {
        self.0.run_property = self.0.run_property.highlight(color);
        self
    }

    pub fn bold(mut self) -> Self {
        self.0.run_property = self.0.run_property.bold();
        self
    }

    pub fn italic(mut self) -> Self {
        self.0.run_property = self.0.run_property.italic();
        self
    }

    pub fn strike(mut self) -> Self {
        self.0.run_property = self.0.run_property.strike();
        self
    }

    pub fn underline(mut self, line_type: &str) -> Self {
        self.0.run_property = self.0.run_property.underline(line_type);
        self
    }

    pub fn vanish(mut self) -> Self {
        self.0.run_property = self.0.run_property.vanish();
        self
    }

    pub fn fonts(mut self, f: RunFonts) -> Self {
        self.0 = self.0.fonts(f.take());
        self
    }

    pub fn spacing(mut self, spacing: i32) -> Self {
        self.0.run_property = self.0.run_property.spacing(spacing);
        self
    }

    pub fn vert_align(mut self, a: VertAlignType) -> Self {
        self.0.run_property = self.0.run_property.vert_align(a);
        self
    }

    pub fn text_border(
        mut self,
        border_type: BorderType,
        size: usize,
        space: usize,
        color: &str,
    ) -> Self {
        let border = TextBorder::new()
            .border_type(border_type)
            .size(size)
            .space(space)
            .color(color);
        self.0.run_property = self.0.run_property.text_border(border);
        self
    }

    pub fn align(mut self, alignment_type: docx_rs::AlignmentType) -> Self {
        self.0.paragraph_property = self.0.paragraph_property.align(alignment_type);
        self
    }

    pub fn indent(
        mut self,
        left: i32,
        special_indent_kind: Option<docx_rs::SpecialIndentKind>,
        special_indent_size: Option<i32>,
    ) -> Self {
        let special_indent = create_special_indent(special_indent_kind, special_indent_size);
        self.0.paragraph_property =
            self.0
                .paragraph_property
                .indent(Some(left), special_indent, None, None);
        self
    }

    pub fn outline_lvl(mut self, l: usize) -> Self {
        self.0.paragraph_property = self.0.paragraph_property.outline_lvl(l);
        self
    }

    // TODO: For now only numbering supported.
    pub fn numbering(mut self, id: usize, level: usize) -> Self {
        let id = docx_rs::NumberingId::new(id);
        let level = docx_rs::IndentLevel::new(level);
        self.0.paragraph_property = self.0.paragraph_property.numbering(id, level);
        self
    }

    pub fn line_spacing(mut self, spacing: LineSpacing) -> Self {
        self.0.paragraph_property = self.0.paragraph_property.line_spacing(spacing.take());
        self
    }

    pub fn keep_next(mut self, v: bool) -> Self {
        self.0.paragraph_property = self.0.paragraph_property.keep_next(v);
        self
    }

    pub fn keep_lines(mut self, v: bool) -> Self {
        self.0.paragraph_property = self.0.paragraph_property.keep_lines(v);
        self
    }

    pub fn page_break_before(mut self, v: bool) -> Self {
        self.0.paragraph_property = self.0.paragraph_property.page_break_before(v);
        self
    }

    pub fn widow_control(mut self, v: bool) -> Self {
        self.0.paragraph_property = self.0.paragraph_property.widow_control(v);
        self
    }

    // pub fn run_property(mut self, p: docx_rs::RunProperty) -> Self {
    //     self.0.run_property = p;
    //     self
    // }

    // pub fn paragraph_property(mut self, p: docx_rs::ParagraphProperty) -> Self {
    //     self.0.paragraph_property = p;
    //     self
    // }

    pub fn table_property(mut self, p: docx_rs::TableProperty) -> Self {
        self.0.table_property = p;
        self
    }

    pub fn table_cell_property(mut self, p: docx_rs::TableCellProperty) -> Self {
        self.0.table_cell_property = p;
        self
    }

    pub fn table_indent(mut self, v: i32) -> Self {
        self.0.table_property = self.0.table_property.indent(v);
        self
    }

    pub fn table_align(mut self, v: docx_rs::TableAlignmentType) -> Self {
        self.0.table_property = self.0.table_property.align(v);
        self
    }

    pub fn set_cell_margins(
        mut self,
        top: usize,
        right: usize,
        bottom: usize,
        left: usize,
    ) -> Self {
        let m = docx_rs::TableCellMargins::new().margin(top, right, bottom, left);
        self.0.table_property = self.0.table_property.set_margins(m);
        self
    }

    pub fn cell_margin_top(mut self, v: usize, t: WidthType) -> Self {
        self.0.table_property = self.0.table_property.cell_margin_top(v, t);
        self
    }

    pub fn cell_margin_right(mut self, v: usize, t: WidthType) -> Self {
        self.0.table_property = self.0.table_property.cell_margin_right(v, t);
        self
    }

    pub fn cell_margin_bottom(mut self, v: usize, t: WidthType) -> Self {
        self.0.table_property = self.0.table_property.cell_margin_bottom(v, t);
        self
    }

    pub fn cell_margin_left(mut self, v: usize, t: WidthType) -> Self {
        self.0.table_property = self.0.table_property.cell_margin_left(v, t);
        self
    }

    pub fn layout(mut self, t: docx_rs::TableLayoutType) -> Self {
        self.0.table_property = self.0.table_property.layout(t);
        self
    }
}

impl Style {
    pub fn take(self) -> docx_rs::Style {
        self.0
    }
}
