use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct PageNumType(docx_rs::PageNumType);

#[wasm_bindgen(js_name = createPageNumType)]
pub fn create_page_num_type(start: Option<u32>, chap_style: Option<String>) -> PageNumType {
    let mut p = docx_rs::PageNumType::new();
    if let Some(start) = start {
        p = p.start(start);
    }
    if let Some(chap_style) = chap_style {
        p = p.chap_style(chap_style);
    }
    PageNumType(p)
}

impl PageNumType {
    pub fn take(self) -> docx_rs::PageNumType {
        self.0
    }
}
