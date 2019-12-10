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

    pub fn add_delete(mut self, delete: Delete) -> Paragraph {
        self.0
            .children
            .push(docx_core::ParagraphChild::Delete(delete.take()));
        self
    }

    pub fn add_bookmark_start(mut self, id: String, name: String) -> Paragraph {
        self.0
            .children
            .push(docx_core::ParagraphChild::BookmarkStart(
                docx_core::BookmarkStart::new(id, name),
            ));
        self
    }

    pub fn add_bookmark_end(mut self, id: String) -> Paragraph {
        self.0.children.push(docx_core::ParagraphChild::BookmarkEnd(
            docx_core::BookmarkEnd::new(id),
        ));
        self
    }

    pub fn add_comment_start(mut self, comment: Comment) -> Paragraph {
        self.0
            .children
            .push(docx_core::ParagraphChild::CommentStart(
                docx_core::CommentRangeStart::new(comment.take()),
            ));
        self
    }

    pub fn add_comment_end(mut self, id: String) -> Paragraph {
        self.0.children.push(docx_core::ParagraphChild::CommentEnd(
            docx_core::CommentRangeEnd::new(id),
        ));
        self
    }

    pub fn align(mut self, alignment_type: docx_core::AlignmentType) -> Paragraph {
        self.0.property = self.0.property.align(alignment_type);
        self
    }

    pub fn style(mut self, style_id: &str) -> Paragraph {
        self.0.property = self.0.property.style(style_id);
        self
    }

    pub fn indent(
        mut self,
        left: usize,
        special_indent_kind: Option<docx_core::SpecialIndentKind>,
        special_indent_size: Option<usize>,
    ) -> Paragraph {
        let mut special_indent = None;
        if let Some(kind) = special_indent_kind {
            let size = if special_indent_size.is_some() {
                special_indent_size.unwrap()
            } else {
                0
            };
            special_indent = match kind {
                docx_core::SpecialIndentKind::FirstLine => {
                    Some(docx_core::SpecialIndentType::FirstLine(size))
                }
                docx_core::SpecialIndentKind::Hanging => {
                    Some(docx_core::SpecialIndentType::Hanging(size))
                }
            }
        }
        self.0.property = self.0.property.indent(left, special_indent);
        self
    }

    pub fn numbering(mut self, id: usize, level: usize) -> Self {
        let id = docx_core::NumberingId::new(id);
        let level = docx_core::IndentLevel::new(level);
        self.0.property = self.0.property.numbering(id, level);
        self
    }
}

impl Paragraph {
    pub fn take(self) -> docx_core::Paragraph {
        self.0
    }
}
