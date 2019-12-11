use super::*;
use docx_core;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Insert(docx_core::Insert);

#[wasm_bindgen(js_name = createInsert)]
pub fn create_insert() -> Insert {
    Insert(docx_core::Insert::new())
}

impl Insert {
    pub fn take(self) -> docx_core::Insert {
        self.0
    }
}
