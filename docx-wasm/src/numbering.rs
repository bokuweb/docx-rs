use super::*;
use docx_core;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Numbering(docx_core::Numbering);

#[wasm_bindgen(js_name = createNumbering)]
pub fn create_numbering(id: usize) -> Numbering {
    Numbering(docx_core::Numbering::new(id))
}

impl Numbering {
    pub fn take(self) -> docx_core::Numbering {
        self.0
    }
}

#[wasm_bindgen]
impl Numbering {
    pub fn add_level(mut self, level: Level) -> Self {
        self.0 = self.0.add_level(level.take());
        self
    }
}
