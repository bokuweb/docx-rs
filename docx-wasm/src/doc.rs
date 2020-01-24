use super::*;
use docx;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Docx(docx::Docx);

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn createDocx() -> Docx {
    Docx(docx::Docx::new())
}

#[wasm_bindgen]
impl Docx {
    pub fn add_paragraph(mut self, p: Paragraph) -> Self {
        self.0 = self.0.add_paragraph(p.take());
        self
    }

    pub fn add_table(mut self, t: Table) -> Docx {
        self.0.document = self.0.document.add_table(t.take());
        self
    }

    pub fn add_numbering(mut self, num: Numbering) -> Docx {
        self.0.numberings = self.0.numberings.add_numbering(num.take());
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
