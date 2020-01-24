use super::*;
use docx_core;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Comment(docx_core::Comment);

#[wasm_bindgen(js_name = createComment)]
pub fn create_comment(id: usize) -> Comment {
    Comment(docx_core::Comment::new(id))
}

impl Comment {
    pub fn take(self) -> docx_core::Comment {
        self.0
    }
}

#[wasm_bindgen]
impl Comment {
    pub fn author(mut self, author: String) -> Comment {
        self.0.author = author;
        self
    }

    pub fn date(mut self, date: String) -> Comment {
        self.0.date = date;
        self
    }

    pub fn paragraph(mut self, p: Paragraph) -> Comment {
        self.0.paragraph = p.take();
        self
    }

    pub fn id(&self) -> usize {
        self.0.id
    }
}
