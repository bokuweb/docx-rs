use super::*;
use docx_rs;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct AbstractNumbering(docx_rs::AbstractNumbering);

#[wasm_bindgen(js_name = createAbstractNumbering)]
pub fn create_abstract_numbering(id: usize) -> AbstractNumbering {
    AbstractNumbering(docx_rs::AbstractNumbering::new(id))
}

impl AbstractNumbering {
    pub fn take(self) -> docx_rs::AbstractNumbering {
        self.0
    }
}

#[wasm_bindgen]
impl AbstractNumbering {
    pub fn add_level(mut self, level: Level) -> Self {
        self.0 = self.0.add_level(level.take());
        self
    }
}
