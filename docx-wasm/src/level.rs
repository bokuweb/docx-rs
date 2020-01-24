use super::*;
use docx;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Level(docx::Level);

#[wasm_bindgen(js_name = createLevel)]
pub fn create_level(id: usize, start: usize, format: &str, text: &str, jc: &str) -> Level {
    let start = docx::Start::new(start);
    let format = docx::NumberFormat::new(format);
    let text = docx::LevelText::new(text);
    let jc = docx::LevelJc::new(jc);
    Level(docx::Level::new(id, start, format, text, jc))
}

impl Level {
    pub fn take(self) -> docx::Level {
        self.0
    }
}

#[wasm_bindgen]
impl Level {
    pub fn indent(
        mut self,
        left: usize,
        special_indent_kind: Option<docx::SpecialIndentKind>,
        special_indent_size: Option<usize>,
    ) -> Self {
        let special_indent = create_special_indent(special_indent_kind, special_indent_size);
        self.0.paragraph_property = self.0.paragraph_property.indent(left, special_indent);
        self
    }
}
