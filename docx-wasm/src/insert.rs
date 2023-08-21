use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Insert(docx_rs::Insert);

#[wasm_bindgen(js_name = createInsert)]
pub fn create_insert(run: Run) -> Insert {
    Insert(docx_rs::Insert::new(run.take()))
}

impl Insert {
    pub fn take(self) -> docx_rs::Insert {
        self.0
    }
}

#[wasm_bindgen]
impl Insert {
    pub fn author(mut self, author: String) -> Insert {
        self.0 = self.0.author(author);
        self
    }

    pub fn date(mut self, date: String) -> Insert {
        self.0 = self.0.date(date);
        self
    }
}
