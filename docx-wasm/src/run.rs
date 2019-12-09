use docx_core;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Run(docx_core::Run);

#[wasm_bindgen(js_name = createRun)]
pub fn create_run() -> Run {
    Run(docx_core::Run::new())
}

#[wasm_bindgen]
impl Run {
    pub fn add_text(mut self, text: &str) -> Self {
        self.0 = self.0.add_text(text);
        self
    }
}

impl Run {
    pub fn take(self) -> docx_core::Run {
        self.0
    }
}
