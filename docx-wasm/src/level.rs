use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Level(docx_rs::Level);

#[wasm_bindgen(js_name = createLevel)]
pub fn create_level(id: usize, start: usize, format: &str, text: &str, jc: &str) -> Level {
    let start = docx_rs::Start::new(start);
    let format = docx_rs::NumberFormat::new(format);
    let text = docx_rs::LevelText::new(text);
    let jc = docx_rs::LevelJc::new(jc);
    Level(docx_rs::Level::new(id, start, format, text, jc))
}

impl Level {
    pub fn take(self) -> docx_rs::Level {
        self.0
    }
}

#[wasm_bindgen]
impl Level {
    pub fn indent(
        mut self,
        left: i32,
        special_indent_kind: Option<docx_rs::SpecialIndentKind>,
        special_indent_size: Option<i32>,
    ) -> Self {
        let special_indent = create_special_indent(special_indent_kind, special_indent_size);
        // end and start_chars is not supported fro wasm for now.
        self.0.paragraph_property =
            self.0
                .paragraph_property
                .indent(Some(left), special_indent, None, None);
        self
    }
}
