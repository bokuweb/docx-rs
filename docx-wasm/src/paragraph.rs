use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ParagraphPropertyChange(docx_rs::ParagraphPropertyChange);

#[wasm_bindgen(js_name = createParagraphPropertyChange)]
pub fn create_paragraph_property_change() -> ParagraphPropertyChange {
    ParagraphPropertyChange(docx_rs::ParagraphPropertyChange::new())
}

#[wasm_bindgen]
impl ParagraphPropertyChange {
    pub fn author(mut self, author: &str) -> Self {
        self.0 = self.0.author(author);
        self
    }

    pub fn date(mut self, date: &str) -> Self {
        self.0 = self.0.date(date);
        self
    }

    // TODO: For now only numbering supported.
    pub fn numbering(mut self, id: usize, level: usize) -> Self {
        let id = docx_rs::NumberingId::new(id);
        let level = docx_rs::IndentLevel::new(level);
        self.0.property = Box::new(self.0.property.numbering(id, level));
        self
    }

    pub fn align(mut self, alignment_type: docx_rs::AlignmentType) -> Self {
        self.0.property = Box::new(self.0.property.align(alignment_type));
        self
    }

    pub fn style(mut self, style_id: &str) -> Self {
        self.0.property = Box::new(self.0.property.style(style_id));
        self
    }

    pub fn indent(
        mut self,
        left: i32,
        special_indent_kind: Option<docx_rs::SpecialIndentKind>,
        special_indent_size: Option<i32>,
    ) -> Self {
        let special_indent = create_special_indent(special_indent_kind, special_indent_size);
        self.0.property = Box::new(
            self.0
                .property
                .indent(Some(left), special_indent, None, None),
        );
        self
    }
}

impl ParagraphPropertyChange {
    pub fn take(self) -> docx_rs::ParagraphPropertyChange {
        self.0
    }
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Paragraph(docx_rs::Paragraph);

#[wasm_bindgen(js_name = createParagraph)]
pub fn create_paragraph() -> Paragraph {
    Paragraph(docx_rs::Paragraph::new())
}

#[wasm_bindgen]
impl Paragraph {
    pub fn add_run(mut self, run: Run) -> Paragraph {
        self.0 = self.0.add_run(run.take());
        self
    }

    pub fn add_hyperlink(mut self, link: Hyperlink) -> Paragraph {
        self.0 = self.0.add_hyperlink(link.take());
        self
    }

    pub fn add_insert(mut self, i: Insert) -> Paragraph {
        self.0
            .children
            .push(docx_rs::ParagraphChild::Insert(i.take()));
        self
    }

    pub fn add_delete(mut self, d: Delete) -> Paragraph {
        self.0
            .children
            .push(docx_rs::ParagraphChild::Delete(d.take()));
        self
    }

    pub fn add_bookmark_start(mut self, id: usize, name: &str) -> Paragraph {
        self.0.children.push(docx_rs::ParagraphChild::BookmarkStart(
            docx_rs::BookmarkStart::new(id, name),
        ));
        self
    }

    pub fn add_bookmark_end(mut self, id: usize) -> Paragraph {
        self.0.children.push(docx_rs::ParagraphChild::BookmarkEnd(
            docx_rs::BookmarkEnd::new(id),
        ));
        self
    }

    pub fn add_comment_start(mut self, comment: Comment) -> Paragraph {
        self.0
            .children
            .push(docx_rs::ParagraphChild::CommentStart(Box::new(
                docx_rs::CommentRangeStart::new(comment.take()),
            )));
        self
    }

    pub fn add_comment_end(mut self, id: usize) -> Paragraph {
        self.0.children.push(docx_rs::ParagraphChild::CommentEnd(
            docx_rs::CommentRangeEnd::new(id),
        ));
        self
    }

    pub fn align(mut self, alignment_type: docx_rs::AlignmentType) -> Paragraph {
        self.0.property = self.0.property.align(alignment_type);
        self
    }

    pub fn text_alignment(mut self, alignment_type: docx_rs::TextAlignmentType) -> Paragraph {
        self.0.property = self.0.property.text_alignment(alignment_type);
        self
    }

    pub fn adjust_right_ind(mut self, v: isize) -> Paragraph {
        self.0.property = self.0.property.adjust_right_ind(v);
        self
    }

    pub fn outline_lvl(mut self, level: usize) -> Paragraph {
        self.0.property = self.0.property.outline_lvl(level);
        self
    }

    pub fn style(mut self, style_id: &str) -> Paragraph {
        self.0.property = self.0.property.style(style_id);
        self
    }

    pub fn indent(
        mut self,
        left: i32,
        special_indent_kind: Option<docx_rs::SpecialIndentKind>,
        special_indent_size: Option<i32>,
        right: Option<i32>,
    ) -> Paragraph {
        let special_indent = create_special_indent(special_indent_kind, special_indent_size);
        self.0.property = self
            .0
            .property
            .indent(Some(left), special_indent, right, None);
        self
    }

    pub fn numbering(mut self, id: usize, level: usize) -> Self {
        let id = docx_rs::NumberingId::new(id);
        let level = docx_rs::IndentLevel::new(level);
        self.0.property = self.0.property.numbering(id, level);
        self
    }

    pub fn size(mut self, size: usize) -> Self {
        self.0 = self.0.size(size);
        self
    }

    pub fn color(mut self, c: &str) -> Self {
        self.0 = self.0.color(c);
        self
    }

    pub fn bold(mut self) -> Self {
        self.0 = self.0.bold();
        self
    }

    pub fn italic(mut self) -> Self {
        self.0 = self.0.italic();
        self
    }

    pub fn fonts(mut self, f: RunFonts) -> Self {
        self.0 = self.0.fonts(f.take());
        self
    }

    pub fn line_spacing(mut self, spacing: LineSpacing) -> Self {
        self.0 = self.0.line_spacing(spacing.take());
        self
    }

    pub fn character_spacing(mut self, spacing: i32) -> Self {
        self.0 = self.0.character_spacing(spacing);
        self
    }

    pub fn keep_next(mut self, v: bool) -> Self {
        self.0 = self.0.keep_next(v);
        self
    }

    pub fn snap_to_grid(mut self, v: bool) -> Self {
        self.0 = self.0.snap_to_grid(v);
        self
    }

    pub fn keep_lines(mut self, v: bool) -> Self {
        self.0 = self.0.keep_lines(v);
        self
    }

    pub fn page_break_before(mut self, v: bool) -> Self {
        self.0 = self.0.page_break_before(v);
        self
    }

    pub fn widow_control(mut self, v: bool) -> Self {
        self.0 = self.0.widow_control(v);
        self
    }

    pub fn delete(mut self, author: &str, date: &str) -> Self {
        self.0 = self.0.delete(author, date);
        self
    }

    pub fn insert(mut self, author: &str, date: &str) -> Self {
        self.0 = self.0.insert(author, date);
        self
    }

    pub fn add_tab(
        mut self,
        val: Option<docx_rs::TabValueType>,
        leader: Option<docx_rs::TabLeaderType>,
        pos: Option<usize>,
    ) -> Self {
        self.0 = self.0.add_tab(docx_rs::Tab { val, leader, pos });
        self
    }

    pub fn paragraph_property_change(mut self, p: ParagraphPropertyChange) -> Self {
        self.0.property = self.0.property.paragraph_property_change(p.take());
        self
    }

    pub fn add_page_num(mut self, p: PageNum) -> Self {
        self.0 = self.0.add_page_num(p.take());
        self
    }

    pub fn add_num_pages(mut self, p: NumPages) -> Self {
        self.0 = self.0.add_num_pages(p.take());
        self
    }

    // frame property
    pub fn wrap(mut self, wrap: &str) -> Self {
        self.0 = self.0.wrap(wrap);
        self
    }

    pub fn v_anchor(mut self, anchor: &str) -> Self {
        self.0 = self.0.v_anchor(anchor);
        self
    }

    pub fn h_anchor(mut self, anchor: &str) -> Self {
        self.0 = self.0.h_anchor(anchor);
        self
    }

    pub fn h_rule(mut self, r: &str) -> Self {
        self.0 = self.0.h_rule(r);
        self
    }

    pub fn x_align(mut self, align: &str) -> Self {
        self.0 = self.0.x_align(align);
        self
    }

    pub fn y_align(mut self, align: &str) -> Self {
        self.0 = self.0.y_align(align);
        self
    }

    pub fn h_space(mut self, x: i32) -> Self {
        self.0 = self.0.h_space(x);
        self
    }

    pub fn v_space(mut self, x: i32) -> Self {
        self.0 = self.0.v_space(x);
        self
    }

    pub fn frame_x(mut self, x: i32) -> Self {
        self.0 = self.0.frame_x(x);
        self
    }

    pub fn frame_y(mut self, y: i32) -> Self {
        self.0 = self.0.frame_y(y);
        self
    }

    pub fn frame_width(mut self, n: u32) -> Self {
        self.0 = self.0.frame_width(n);
        self
    }

    pub fn frame_height(mut self, n: u32) -> Self {
        self.0 = self.0.frame_height(n);
        self
    }
}

impl Paragraph {
    pub fn take(self) -> docx_rs::Paragraph {
        self.0
    }
}
