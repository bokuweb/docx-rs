use super::*;
use docx_core;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Paragraph(docx_core::Paragraph);

#[wasm_bindgen(js_name = createParagraph)]
pub fn create_paragraph() -> Paragraph {
    Paragraph(docx_core::Paragraph::new())
}

#[wasm_bindgen]
impl Paragraph {
    pub fn add_run(mut self, run: Run) -> Paragraph {
        self.0 = self.0.add_run(run.take());
        self
    }

    pub fn add_insert(mut self, insert: Insert) -> Paragraph {
        self.0
            .children
            .push(docx_core::ParagraphChild::Insert(insert.take()));
        self
    }
}

impl Paragraph {
    pub fn take(self) -> docx_core::Paragraph {
        self.0
    }
}
