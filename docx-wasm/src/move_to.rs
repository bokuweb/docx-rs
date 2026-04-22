use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct MoveTo(docx_rs::MoveTo);

#[wasm_bindgen(js_name = createMoveTo)]
pub fn create_move_to(run: Run) -> MoveTo {
    MoveTo(docx_rs::MoveTo::new(run.take()))
}

impl MoveTo {
    pub fn take(self) -> docx_rs::MoveTo {
        self.0
    }
}

#[wasm_bindgen]
impl MoveTo {
    pub fn author(mut self, author: String) -> MoveTo {
        self.0 = self.0.author(author);
        self
    }

    pub fn date(mut self, date: String) -> MoveTo {
        self.0 = self.0.date(date);
        self
    }
}
