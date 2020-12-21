use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Comment(docx_rs::Comment);

#[wasm_bindgen(js_name = createComment)]
pub fn create_comment(id: usize) -> Comment {
    Comment(docx_rs::Comment::new(id))
}

impl Comment {
    pub fn take(self) -> docx_rs::Comment {
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

    pub fn add_paragraph(mut self, p: Paragraph) -> Comment {
        self.0 = self.0.add_paragraph(p.take());
        self
    }

    pub fn parent_comment_id(mut self, id: usize) -> Comment {
        self.0 = self.0.parent_comment_id(id);
        self
    }

    pub fn id(&self) -> usize {
        self.0.id
    }
}
