use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Footer(docx_rs::Footer);

#[wasm_bindgen(js_name = createFooter)]
pub fn create_footer() -> Footer {
    Footer(docx_rs::Footer::new())
}

impl Footer {
    pub fn take(self) -> docx_rs::Footer {
        self.0
    }
}

#[wasm_bindgen]
impl Footer {
    pub fn add_paragraph(mut self, p: Paragraph) -> Self {
        self.0 = self.0.add_paragraph(p.take());
        self
    }

    pub fn add_table(mut self, t: Table) -> Self {
        self.0 = self.0.add_table(t.take());
        self
    }

    pub fn add_page_num(mut self, t: PageNum) -> Self {
        self.0 = self.0.add_page_num(t.take());
        self
    }

    pub fn add_num_pages(mut self, t: NumPages) -> Self {
        self.0 = self.0.add_num_pages(t.take());
        self
    }
}
