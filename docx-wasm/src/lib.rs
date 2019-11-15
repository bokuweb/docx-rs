use docx_core;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Docx(docx_core::Docx<'static>);

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn createDocx() -> Docx {
    Docx(docx_core::Docx::new())
}

#[wasm_bindgen]
impl Docx {
    pub fn add_paragraph(mut self) -> Self {
        self.0 = self.0.add_paragraph(
            docx_core::Paragraph::new().add_run(docx_core::Run::new().add_text("Hello")),
        );
        self
    }

    pub fn build(&self) -> Result<Vec<u8>, JsValue> {
        let buf = Vec::new();
        let mut cur = std::io::Cursor::new(buf);
        let res = self.0.build().pack(&mut cur);
        if res.is_err() {
            return Err(format!("{:?}", res).into());
        }
        Ok(cur.into_inner())
    }

    pub fn test(&self, t: docx_core::StyleType) {
        ()
    }
}
