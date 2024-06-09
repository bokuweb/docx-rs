use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct PageNum(docx_rs::PageNum);

#[wasm_bindgen(js_name = createPageNum)]
pub fn create_page_num() -> PageNum {
    PageNum(docx_rs::PageNum::new())
}

impl PageNum {
    pub fn take(self) -> docx_rs::PageNum {
        self.0
    }
}
