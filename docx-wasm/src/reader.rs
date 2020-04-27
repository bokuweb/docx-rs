use wasm_bindgen::prelude::*;

#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn readDocx(buf: &[u8]) -> Result<String, JsValue> {
    let mut d = docx_rs::read_docx(buf);
    match d {
        Ok(ref mut d) => Ok(d.json()),
        Err(e) => Err(e.to_string().into()),
    }
}
