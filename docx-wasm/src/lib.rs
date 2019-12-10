mod comment;
mod delete;
mod insert;
mod paragraph;
mod run;

use docx_core;
use wasm_bindgen::prelude::*;

pub use comment::*;
pub use delete::*;
pub use insert::*;
pub use paragraph::*;
pub use run::*;

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
    pub fn add_paragraph(mut self, p: Paragraph) -> Self {
        self.0 = self.0.add_paragraph(p.take());
        self
    }

    pub fn build(&mut self) -> Result<Vec<u8>, JsValue> {
        let buf = Vec::new();
        let mut cur = std::io::Cursor::new(buf);
        let res = self.0.build().pack(&mut cur);
        if res.is_err() {
            return Err(format!("{:?}", res).into());
        }
        Ok(cur.into_inner())
    }
}
