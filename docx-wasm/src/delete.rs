use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Delete(docx_rs::Delete);

#[wasm_bindgen(js_name = createDelete)]
pub fn create_delete(run: Run) -> Delete {
    Delete(docx_rs::Delete::new().add_run(run.take()))
}

impl Delete {
    pub fn take(self) -> docx_rs::Delete {
        self.0
    }
}

#[wasm_bindgen]
impl Delete {
    pub fn author(mut self, author: String) -> Delete {
        self.0 = self.0.author(author);
        self
    }

    pub fn date(mut self, date: String) -> Delete {
        self.0 = self.0.date(date);
        self
    }
}
