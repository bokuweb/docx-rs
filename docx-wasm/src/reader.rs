use docx_rs;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name=readError)]
#[allow(non_snake_case)]
pub fn readDocx(buf: &[u8]) -> Result<String, JsValue> {
    let mut d = docx_rs::read_docx(buf);
    match d {
        Ok(ref mut d) => Ok(d.json()),
        Err(e) => Err(e.to_string().into()),
    }
}
