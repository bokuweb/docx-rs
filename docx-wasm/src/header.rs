use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Header(docx_rs::Header);

#[wasm_bindgen(js_name = createHeader)]
pub fn create_footer() -> Header {
    Header(docx_rs::Header::new())
}

impl Header {
    pub fn take(self) -> docx_rs::Header {
        self.0
    }
}

#[wasm_bindgen]
impl Header {
    pub fn add_paragraph(mut self, p: Paragraph) -> Self {
        self.0 = self.0.add_paragraph(p.take());
        self
    }

    pub fn add_table(mut self, t: Table) -> Self {
        self.0 = self.0.add_table(t.take());
        self
    }
}
