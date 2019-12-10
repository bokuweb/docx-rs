use super::*;
use docx_core;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Delete(docx_core::Delete);

#[wasm_bindgen(js_name = createDelete)]
pub fn create_delete() -> Delete {
    Delete(docx_core::Delete::new())
}

impl Delete {
    pub fn take(self) -> docx_core::Delete {
        self.0
    }
}
