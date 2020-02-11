use docx_rs;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Numbering(docx_rs::Numbering);

#[wasm_bindgen(js_name = createNumbering)]
pub fn create_numbering(id: usize, abstract_num_id: usize) -> Numbering {
    Numbering(docx_rs::Numbering::new(id, abstract_num_id))
}

impl Numbering {
    pub fn take(self) -> docx_rs::Numbering {
        self.0
    }
}
