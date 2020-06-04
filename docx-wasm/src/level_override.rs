use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct LevelOverride(docx_rs::LevelOverride);

#[wasm_bindgen(js_name = createLevelOverride)]
pub fn create_level_override(level: usize) -> LevelOverride {
    LevelOverride(docx_rs::LevelOverride::new(level))
}

impl LevelOverride {
    pub fn take(self) -> docx_rs::LevelOverride {
        self.0
    }
}

#[wasm_bindgen]
impl LevelOverride {
    pub fn start(mut self, start: usize) -> Self {
        self.0 = self.0.start(start);
        self
    }
}
