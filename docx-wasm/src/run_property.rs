use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct RunProperty(docx_rs::RunProperty);

#[wasm_bindgen(js_name = createRunProperty)]
pub fn create_run_property() -> RunProperty {
    RunProperty(docx_rs::RunProperty::new())
}

#[wasm_bindgen]
impl RunProperty {
    pub fn size(mut self, size: usize) -> Self {
        self.0 = self.0.size(size);
        self
    }

    pub fn color(mut self, c: &str) -> Self {
        self.0 = self.0.color(c);
        self
    }

    pub fn bold(mut self) -> Self {
        self.0 = self.0.bold();
        self
    }

    pub fn italic(mut self) -> Self {
        self.0 = self.0.italic();
        self
    }

    pub fn strike(mut self) -> Self {
        self.0 = self.0.strike();
        self
    }

    pub fn fonts(mut self, f: RunFonts) -> Self {
        self.0 = self.0.fonts(f.take());
        self
    }

    pub fn underline(mut self, line_type: &str) -> Self {
        self.0 = self.0.underline(line_type);
        self
    }

    pub fn vanish(mut self) -> Self {
        self.0 = self.0.vanish();
        self
    }

    pub fn spec_vanish(mut self) -> Self {
        self.0 = self.0.spec_vanish();
        self
    }

    pub fn character_spacing(mut self, spacing: i32) -> Self {
        self.0 = self.0.character_spacing(spacing);
        self
    }

    pub fn vert_align(mut self, a: docx_rs::VertAlignType) -> Self {
        self.0 = self.0.vert_align(a);
        self
    }

    pub fn delete(mut self, author: &str, date: &str) -> Self {
        self.0 = self
            .0
            .delete(docx_rs::Delete::new().author(author).date(date));
        self
    }

    pub fn insert(mut self, author: &str, date: &str) -> Self {
        self.0 = self
            .0
            .insert(docx_rs::Insert::new_with_empty().author(author).date(date));
        self
    }

    pub fn style(mut self, style_id: &str) -> Self {
        self.0 = self.0.style(style_id);
        self
    }

    pub fn highlight(mut self, color: &str) -> Self {
        self.0 = self.0.highlight(color);
        self
    }

    pub fn text_border(
        mut self,
        border_type: docx_rs::BorderType,
        size: usize,
        space: usize,
        color: &str,
    ) -> Self {
        let border = docx_rs::TextBorder::new()
            .border_type(border_type)
            .size(size)
            .space(space)
            .color(color);
        self.0 = self.0.text_border(border);
        self
    }
}

impl RunProperty {
    pub fn take(self) -> docx_rs::RunProperty {
        self.0
    }
}