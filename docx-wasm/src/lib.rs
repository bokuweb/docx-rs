use docx_core;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Docx(docx_core::Docx);

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn createDocx() -> Docx {
    Docx(docx_core::Docx::new())
}

#[wasm_bindgen]
impl Docx {
    pub fn add_paragraph(mut self) -> Self {
        self.0 = self
            .0
            .add_paragraph(docx_core::Paragraph::new().add_run(docx_core::Run::new("Hello")));
        self
    }

    pub fn build(&self) -> Vec<u8> {
        let buf = Vec::new();
        let mut cur = std::io::Cursor::new(buf);
        let b = self.0.build();
        docx_core::zip(&mut cur, b).unwrap();
        cur.into_inner()
    }

    pub fn test(&self, t: docx_core::StyleType) {
        ()
    }
}
