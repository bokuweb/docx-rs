use wasm_bindgen::prelude::*;

use super::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Hyperlink(docx_rs::Hyperlink);

#[wasm_bindgen(js_name = createHyperlink)]
pub fn create_hyperlink(v: &str, t: docx_rs::HyperlinkType) -> Hyperlink {
    Hyperlink(docx_rs::Hyperlink::new(v, t))
}

#[wasm_bindgen]
impl Hyperlink {
    pub fn add_run(mut self, run: Run) -> Self {
        self.0 = self.0.add_run(run.take());
        self
    }

    pub fn add_insert(mut self, i: Insert) -> Self {
        self.0
            .children
            .push(docx_rs::ParagraphChild::Insert(i.take()));
        self
    }

    pub fn add_delete(mut self, d: Delete) -> Self {
        self.0
            .children
            .push(docx_rs::ParagraphChild::Delete(d.take()));
        self
    }

    pub fn add_bookmark_start(mut self, id: usize, name: &str) -> Self {
        self.0.children.push(docx_rs::ParagraphChild::BookmarkStart(
            docx_rs::BookmarkStart::new(id, name),
        ));
        self
    }

    pub fn add_bookmark_end(mut self, id: usize) -> Self {
        self.0.children.push(docx_rs::ParagraphChild::BookmarkEnd(
            docx_rs::BookmarkEnd::new(id),
        ));
        self
    }

    pub fn add_comment_start(mut self, comment: Comment) -> Self {
        self.0
            .children
            .push(docx_rs::ParagraphChild::CommentStart(Box::new(
                docx_rs::CommentRangeStart::new(comment.take()),
            )));
        self
    }

    pub fn add_comment_end(mut self, id: usize) -> Self {
        self.0.children.push(docx_rs::ParagraphChild::CommentEnd(
            docx_rs::CommentRangeEnd::new(id),
        ));
        self
    }
}

impl Hyperlink {
    pub fn take(self) -> docx_rs::Hyperlink {
        self.0
    }
}
