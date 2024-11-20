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

    pub fn fonts(mut self, f: RunFonts) -> Self {
        self.0 = self.0.fonts(f.take());
        self
    }

    pub fn character_spacing(mut self, spacing: i32) -> Self {
        self.0 = self.0.character_spacing(spacing);
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
}

impl RunProperty {
    pub fn take(self) -> docx_rs::RunProperty {
        self.0
    }
}
