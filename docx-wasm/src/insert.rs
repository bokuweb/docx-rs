use super::*;
use docx_core;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Insert(docx_core::Insert);

#[wasm_bindgen(js_name = createInsert)]
pub fn create_insert() -> Insert {
    Insert(docx_core::Insert::new())
}

impl Insert {
    pub fn take(self) -> docx_core::Insert {
        self.0
    }

    pub fn run(mut self, run: Run) -> Insert {
        self.0.run = run.take();
        self
    }

    pub fn author(mut self, author: String) -> Insert {
        self.0.author = author;
        self
    }

    pub fn date(mut self, date: String) -> Insert {
        self.0.date = date;
        self
    }
}
