use super::*;
use docx_core;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Delete(docx_core::Delete);

#[wasm_bindgen(js_name = createDelete)]
pub fn create_delete() -> Delete {
    Delete(docx_core::Delete::new())
}

impl Delete {
    pub fn take(self) -> docx_core::Delete {
        self.0
    }

    pub fn run(mut self, run: Run) -> Delete {
        self.0.run = run.take();
        self
    }

    pub fn author(mut self, author: String) -> Delete {
        self.0.author = author;
        self
    }

    pub fn date(mut self, date: String) -> Delete {
        self.0.date = date;
        self
    }
}
