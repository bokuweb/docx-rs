use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct PageMargin(docx_rs::PageMargin);

#[wasm_bindgen(js_name = createPageMargin)]
pub fn create_page_margin() -> PageMargin {
    PageMargin(docx_rs::PageMargin::new())
}

impl PageMargin {
    pub fn take(self) -> docx_rs::PageMargin {
        self.0
    }
}

#[wasm_bindgen]
impl PageMargin {
    pub fn top(self, v: i32) -> PageMargin {
        PageMargin(docx_rs::PageMargin { top: v, ..self.0 })
    }

    pub fn left(self, v: i32) -> PageMargin {
        PageMargin(docx_rs::PageMargin { left: v, ..self.0 })
    }

    pub fn bottom(self, v: i32) -> PageMargin {
        PageMargin(docx_rs::PageMargin {
            bottom: v,
            ..self.0
        })
    }

    pub fn right(self, v: i32) -> PageMargin {
        PageMargin(docx_rs::PageMargin { right: v, ..self.0 })
    }

    pub fn header(self, v: i32) -> PageMargin {
        PageMargin(docx_rs::PageMargin {
            header: v,
            ..self.0
        })
    }

    pub fn footer(self, v: i32) -> PageMargin {
        PageMargin(docx_rs::PageMargin {
            footer: v,
            ..self.0
        })
    }

    pub fn gutter(self, v: i32) -> PageMargin {
        PageMargin(docx_rs::PageMargin {
            gutter: v,
            ..self.0
        })
    }
}
