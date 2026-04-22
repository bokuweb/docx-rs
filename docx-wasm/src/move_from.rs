use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct MoveFrom(docx_rs::MoveFrom);

#[wasm_bindgen(js_name = createMoveFrom)]
pub fn create_move_from(run: Run) -> MoveFrom {
    MoveFrom(docx_rs::MoveFrom::new().add_run(run.take()))
}

impl MoveFrom {
    pub fn take(self) -> docx_rs::MoveFrom {
        self.0
    }
}

#[wasm_bindgen]
impl MoveFrom {
    pub fn author(mut self, author: String) -> MoveFrom {
        self.0 = self.0.author(author);
        self
    }

    pub fn date(mut self, date: String) -> MoveFrom {
        self.0 = self.0.date(date);
        self
    }
}
