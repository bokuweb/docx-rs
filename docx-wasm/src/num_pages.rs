use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct NumPages(docx_rs::NumPages);

#[wasm_bindgen(js_name = createNumPages)]
pub fn create_num_pages() -> NumPages {
    NumPages(docx_rs::NumPages::new())
}

impl NumPages {
    pub fn take(self) -> docx_rs::NumPages {
        self.0
    }
}
