use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;

use super::*;
use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;

#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Paragraph {
    pub id: String,
    pub children: Vec<ParagraphChild>,
    pub property: ParagraphProperty,
    pub has_numbering: bool,
}

impl Default for Paragraph {
    fn default() -> Self {
        Self {
            id: crate::generate_para_id(),
            children: Vec::new(),
            property: ParagraphProperty::new(),
            has_numbering: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParagraphChild {
    Run(Box<Run>),
    Insert(Insert),
    Delete(Delete),
    BookmarkStart(BookmarkStart),
    Hyperlink(Hyperlink),
    BookmarkEnd(BookmarkEnd),
    CommentStart(Box<CommentRangeStart>),
    CommentEnd(CommentRangeEnd),
    StructuredDataTag(Box<StructuredDataTag>),
    PageNum(Box<PageNum>),
    NumPages(Box<NumPages>),
}

impl BuildXML for ParagraphChild {
    fn build(&self) -> Vec<u8> {
        match self {
            ParagraphChild::Run(v) => v.build(),
            ParagraphChild::Insert(v) => v.build(),
            ParagraphChild::Delete(v) => v.build(),
            ParagraphChild::Hyperlink(v) => v.build(),
            ParagraphChild::BookmarkStart(v) => v.build(),
            ParagraphChild::BookmarkEnd(v) => v.build(),
            ParagraphChild::CommentStart(v) => v.build(),
            ParagraphChild::CommentEnd(v) => v.build(),
            ParagraphChild::StructuredDataTag(v) => v.build(),
            ParagraphChild::PageNum(v) => v.build(),
            ParagraphChild::NumPages(v) => v.build(),
        }
    }
}

impl Serialize for ParagraphChild {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            ParagraphChild::Run(ref r) => {
                let mut t = serializer.serialize_struct("Run", 2)?;
                t.serialize_field("type", "run")?;
                t.serialize_field("data", r)?;
                t.end()
            }
            ParagraphChild::Insert(ref r) => {
                let mut t = serializer.serialize_struct("Insert", 2)?;
                t.serialize_field("type", "insert")?;
                t.serialize_field("data", r)?;
                t.end()
            }
            ParagraphChild::Delete(ref r) => {
                let mut t = serializer.serialize_struct("Delete", 2)?;
                t.serialize_field("type", "delete")?;
                t.serialize_field("data", r)?;
                t.end()
            }
            ParagraphChild::Hyperlink(ref r) => {
                let mut t = serializer.serialize_struct("hyperlink", 2)?;
                t.serialize_field("type", "hyperlink")?;
                t.serialize_field("data", r)?;
                t.end()
            }
            ParagraphChild::BookmarkStart(ref r) => {
                let mut t = serializer.serialize_struct("BookmarkStart", 2)?;
                t.serialize_field("type", "bookmarkStart")?;
                t.serialize_field("data", r)?;
                t.end()
            }
            ParagraphChild::BookmarkEnd(ref r) => {
                let mut t = serializer.serialize_struct("BookmarkEnd", 2)?;
                t.serialize_field("type", "bookmarkEnd")?;
                t.serialize_field("data", r)?;
                t.end()
            }
            ParagraphChild::CommentStart(ref r) => {
                let mut t = serializer.serialize_struct("CommentRangeStart", 2)?;
                t.serialize_field("type", "commentRangeStart")?;
                t.serialize_field("data", r)?;
                t.end()
            }
            ParagraphChild::CommentEnd(ref r) => {
                let mut t = serializer.serialize_struct("CommentRangeEnd", 2)?;
                t.serialize_field("type", "commentRangeEnd")?;
                t.serialize_field("data", r)?;
                t.end()
            }
            ParagraphChild::StructuredDataTag(ref r) => {
                let mut t = serializer.serialize_struct("StructuredDataTag", 2)?;
                t.serialize_field("type", "structuredDataTag")?;
                t.serialize_field("data", r)?;
                t.end()
            }
            ParagraphChild::PageNum(ref r) => {
                let mut t = serializer.serialize_struct("PageNum", 2)?;
                t.serialize_field("type", "pageNum")?;
                t.serialize_field("data", r)?;
                t.end()
            }
            ParagraphChild::NumPages(ref r) => {
                let mut t = serializer.serialize_struct("NumPages", 2)?;
                t.serialize_field("type", "numPages")?;
                t.serialize_field("data", r)?;
                t.end()
            }
        }
    }
}

impl Paragraph {
    pub fn new() -> Paragraph {
        Default::default()
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }

    pub fn children(&self) -> &Vec<ParagraphChild> {
        &self.children
    }

    pub fn add_run(mut self, run: Run) -> Paragraph {
        self.children.push(ParagraphChild::Run(Box::new(run)));
        self
    }

    pub(crate) fn unshift_run(mut self, run: Run) -> Paragraph {
        self.children.insert(0, ParagraphChild::Run(Box::new(run)));
        self
    }

    pub(crate) fn wrap_by_bookmark(mut self, id: usize, name: impl Into<String>) -> Paragraph {
        self.children.insert(
            0,
            ParagraphChild::BookmarkStart(BookmarkStart::new(id, name)),
        );
        self.children
            .push(ParagraphChild::BookmarkEnd(BookmarkEnd::new(id)));

        self
    }

    pub fn add_hyperlink(mut self, link: Hyperlink) -> Self {
        self.children.push(ParagraphChild::Hyperlink(link));
        self
    }

    pub fn add_structured_data_tag(mut self, t: StructuredDataTag) -> Self {
        self.children
            .push(ParagraphChild::StructuredDataTag(Box::new(t)));
        self
    }

    pub fn add_insert(mut self, insert: Insert) -> Paragraph {
        self.children.push(ParagraphChild::Insert(insert));
        self
    }

    pub fn add_delete(mut self, delete: Delete) -> Paragraph {
        self.children.push(ParagraphChild::Delete(delete));
        self
    }

    pub fn add_bookmark_start(mut self, id: usize, name: impl Into<String>) -> Paragraph {
        self.children
            .push(ParagraphChild::BookmarkStart(BookmarkStart::new(id, name)));
        self
    }

    pub fn add_bookmark_end(mut self, id: usize) -> Paragraph {
        self.children
            .push(ParagraphChild::BookmarkEnd(BookmarkEnd::new(id)));
        self
    }

    pub fn add_comment_start(mut self, comment: Comment) -> Paragraph {
        self.children.push(ParagraphChild::CommentStart(Box::new(
            CommentRangeStart::new(comment),
        )));
        self
    }

    pub fn add_comment_end(mut self, id: usize) -> Paragraph {
        self.children
            .push(ParagraphChild::CommentEnd(CommentRangeEnd::new(id)));
        self
    }

    pub fn align(mut self, alignment_type: AlignmentType) -> Paragraph {
        self.property = self.property.align(alignment_type);
        self
    }

    pub fn style(mut self, style_id: &str) -> Paragraph {
        self.property = self.property.style(style_id);
        self
    }

    pub fn snap_to_grid(mut self, v: bool) -> Self {
        self.property = self.property.snap_to_grid(v);
        self
    }

    pub fn keep_next(mut self, v: bool) -> Self {
        self.property = self.property.keep_next(v);
        self
    }

    pub fn keep_lines(mut self, v: bool) -> Self {
        self.property = self.property.keep_lines(v);
        self
    }

    pub fn outline_lvl(mut self, v: usize) -> Self {
        self.property = self.property.outline_lvl(v);
        self
    }

    pub fn page_break_before(mut self, v: bool) -> Self {
        self.property = self.property.page_break_before(v);
        self
    }

    pub fn widow_control(mut self, v: bool) -> Self {
        self.property = self.property.widow_control(v);
        self
    }

    pub fn section_property(mut self, s: SectionProperty) -> Self {
        self.property = self.property.section_property(s);
        self
    }

    pub fn add_tab(mut self, t: Tab) -> Self {
        self.property = self.property.add_tab(t);
        self
    }

    pub fn tabs(mut self, tabs: &[Tab]) -> Self {
        for tab in tabs {
            self.property = self.property.add_tab(tab.clone());
        }
        self
    }

    pub fn indent(
        mut self,
        left: Option<i32>,
        special_indent: Option<SpecialIndentType>,
        end: Option<i32>,
        start_chars: Option<i32>,
    ) -> Paragraph {
        self.property = self.property.indent(left, special_indent, end, start_chars);
        self
    }

    pub fn hanging_chars(mut self, chars: i32) -> Paragraph {
        self.property = self.property.hanging_chars(chars);
        self
    }

    pub fn first_line_chars(mut self, chars: i32) -> Paragraph {
        self.property = self.property.first_line_chars(chars);
        self
    }

    pub fn numbering(mut self, id: NumberingId, level: IndentLevel) -> Self {
        self.property = self.property.numbering(id, level);
        self.has_numbering = true;
        self
    }

    pub fn size(mut self, size: usize) -> Self {
        self.property.run_property = self.property.run_property.size(size);
        self
    }

    pub fn color(mut self, c: impl Into<String>) -> Self {
        self.property.run_property = self.property.run_property.color(c);
        self
    }

    pub fn bold(mut self) -> Self {
        self.property.run_property = self.property.run_property.bold();
        self
    }

    pub fn italic(mut self) -> Self {
        self.property.run_property = self.property.run_property.italic();
        self
    }

    pub fn fonts(mut self, f: RunFonts) -> Self {
        self.property.run_property = self.property.run_property.fonts(f);
        self
    }

    pub fn run_property(mut self, p: RunProperty) -> Self {
        self.property.run_property = p;
        self
    }

    pub fn line_spacing(mut self, spacing: LineSpacing) -> Self {
        self.property = self.property.line_spacing(spacing);
        self
    }

    pub fn character_spacing(mut self, spacing: i32) -> Self {
        self.property = self.property.character_spacing(spacing);
        self
    }

    pub fn delete(mut self, author: impl Into<String>, date: impl Into<String>) -> Self {
        self.property.run_property.del = Some(Delete::new().author(author).date(date));
        self
    }

    pub fn insert(mut self, author: impl Into<String>, date: impl Into<String>) -> Self {
        self.property.run_property.ins = Some(Insert::new_with_empty().author(author).date(date));
        self
    }

    pub fn paragraph_property_change(mut self, p: ParagraphPropertyChange) -> Self {
        self.property = self.property.paragraph_property_change(p);
        self
    }

    pub fn raw_text(&self) -> String {
        let mut s = "".to_string();
        // For now support only run and ins.
        for c in self.children.iter() {
            match c {
                ParagraphChild::Insert(i) => {
                    for c in i.children.iter() {
                        if let InsertChild::Run(r) = c {
                            for c in r.children.iter() {
                                if let RunChild::Text(t) = c {
                                    s.push_str(&t.text);
                                }
                            }
                        }
                    }
                }
                ParagraphChild::Run(run) => {
                    for c in run.children.iter() {
                        if let RunChild::Text(t) = c {
                            s.push_str(&t.text);
                        }
                    }
                }
                _ => {}
            }
        }
        s
    }

    pub fn add_page_num(mut self, p: PageNum) -> Self {
        self.children.push(ParagraphChild::PageNum(Box::new(p)));
        self
    }

    pub fn add_num_pages(mut self, p: NumPages) -> Self {
        self.children.push(ParagraphChild::NumPages(Box::new(p)));
        self
    }

    // frameProperty
    pub fn wrap(mut self, wrap: impl Into<String>) -> Self {
        self.property.frame_property = Some(FrameProperty {
            wrap: Some(wrap.into()),
            ..self.property.frame_property.unwrap_or_default()
        });
        self
    }

    pub fn v_anchor(mut self, anchor: impl Into<String>) -> Self {
        self.property.frame_property = Some(FrameProperty {
            v_anchor: Some(anchor.into()),
            ..self.property.frame_property.unwrap_or_default()
        });
        self
    }

    pub fn h_anchor(mut self, anchor: impl Into<String>) -> Self {
        self.property.frame_property = Some(FrameProperty {
            h_anchor: Some(anchor.into()),
            ..self.property.frame_property.unwrap_or_default()
        });
        self
    }

    pub fn h_rule(mut self, r: impl Into<String>) -> Self {
        self.property.frame_property = Some(FrameProperty {
            h_rule: Some(r.into()),
            ..self.property.frame_property.unwrap_or_default()
        });
        self
    }

    pub fn x_align(mut self, align: impl Into<String>) -> Self {
        self.property.frame_property = Some(FrameProperty {
            x_align: Some(align.into()),
            ..self.property.frame_property.unwrap_or_default()
        });
        self
    }

    pub fn y_align(mut self, align: impl Into<String>) -> Self {
        self.property.frame_property = Some(FrameProperty {
            y_align: Some(align.into()),
            ..self.property.frame_property.unwrap_or_default()
        });
        self
    }

    pub fn h_space(mut self, x: i32) -> Self {
        self.property.frame_property = Some(FrameProperty {
            h_space: Some(x),
            ..self.property.frame_property.unwrap_or_default()
        });
        self
    }

    pub fn v_space(mut self, x: i32) -> Self {
        self.property.frame_property = Some(FrameProperty {
            v_space: Some(x),
            ..self.property.frame_property.unwrap_or_default()
        });
        self
    }

    pub fn frame_x(mut self, x: i32) -> Self {
        self.property.frame_property = Some(FrameProperty {
            x: Some(x),
            ..self.property.frame_property.unwrap_or_default()
        });
        self
    }

    pub fn frame_y(mut self, y: i32) -> Self {
        self.property.frame_property = Some(FrameProperty {
            y: Some(y),
            ..self.property.frame_property.unwrap_or_default()
        });
        self
    }

    pub fn frame_width(mut self, n: u32) -> Self {
        self.property.frame_property = Some(FrameProperty {
            w: Some(n),
            ..self.property.frame_property.unwrap_or_default()
        });
        self
    }

    pub fn frame_height(mut self, n: u32) -> Self {
        self.property.frame_property = Some(FrameProperty {
            h: Some(n),
            ..self.property.frame_property.unwrap_or_default()
        });
        self
    }
}

impl BuildXML for Paragraph {
    fn build(&self) -> Vec<u8> {
        XMLBuilder::new(Vec::new())
            .open_paragraph(&self.id)
            .add_child(&self.property)
            .add_children(&self.children)
            .close()
            .into_inner()
    }
}

impl BuildXML for Box<Paragraph> {
    fn build(&self) -> Vec<u8> {
        Paragraph::build(self)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_paragraph() {
        let b = Paragraph::new()
            .add_run(Run::new().add_text("Hello"))
            .build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:p w14:paraId="12345678"><w:pPr><w:rPr /></w:pPr><w:r><w:rPr /><w:t xml:space="preserve">Hello</w:t></w:r></w:p>"#
        );
    }

    #[test]
    fn test_bookmark() {
        let b = Paragraph::new()
            .add_bookmark_start(0, "article")
            .add_run(Run::new().add_text("Hello"))
            .add_bookmark_end(0)
            .build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:p w14:paraId="12345678"><w:pPr><w:rPr /></w:pPr><w:bookmarkStart w:id="0" w:name="article" /><w:r><w:rPr /><w:t xml:space="preserve">Hello</w:t></w:r><w:bookmarkEnd w:id="0" /></w:p>"#
        );
    }

    #[test]
    fn test_comment() {
        let b = Paragraph::new()
            .add_comment_start(Comment::new(1))
            .add_run(Run::new().add_text("Hello"))
            .add_comment_end(1)
            .build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:p w14:paraId="12345678"><w:pPr><w:rPr /></w:pPr><w:commentRangeStart w:id="1" /><w:r><w:rPr /><w:t xml:space="preserve">Hello</w:t></w:r><w:r>
  <w:rPr />
</w:r>
<w:commentRangeEnd w:id="1" />
<w:r>
  <w:commentReference w:id="1" />
</w:r></w:p>"#
        );
    }

    #[test]
    fn test_numbering() {
        let b = Paragraph::new()
            .add_run(Run::new().add_text("Hello"))
            .numbering(NumberingId::new(0), IndentLevel::new(1))
            .build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:p w14:paraId="12345678"><w:pPr><w:rPr /><w:numPr><w:numId w:val="0" /><w:ilvl w:val="1" /></w:numPr></w:pPr><w:r><w:rPr /><w:t xml:space="preserve">Hello</w:t></w:r></w:p>"#
        );
    }

    #[test]
    fn test_line_spacing_and_character_spacing() {
        let spacing = LineSpacing::new()
            .line_rule(LineSpacingType::Auto)
            .before(20)
            .after(30)
            .line(200);
        let b = Paragraph::new()
            .line_spacing(spacing)
            .add_run(Run::new().add_text("Hello"))
            .build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:p w14:paraId="12345678"><w:pPr><w:rPr /><w:spacing w:before="20" w:after="30" w:line="200" w:lineRule="auto" /></w:pPr><w:r><w:rPr /><w:t xml:space="preserve">Hello</w:t></w:r></w:p>"#
        );
    }

    #[test]
    fn test_paragraph_run_json() {
        let run = Run::new().add_text("Hello");
        let p = Paragraph::new().add_run(run);
        assert_eq!(
            serde_json::to_string(&p).unwrap(),
            r#"{"id":"12345678","children":[{"type":"run","data":{"runProperty":{},"children":[{"type":"text","data":{"preserveSpace":true,"text":"Hello"}}]}}],"property":{"runProperty":{},"tabs":[]},"hasNumbering":false}"#,
        );
    }

    #[test]
    fn test_paragraph_insert_json() {
        let run = Run::new().add_text("Hello");
        let ins = Insert::new(run);
        let p = Paragraph::new().add_insert(ins);
        assert_eq!(
            serde_json::to_string(&p).unwrap(),
            r#"{"id":"12345678","children":[{"type":"insert","data":{"children":[{"type":"run","data":{"runProperty":{},"children":[{"type":"text","data":{"preserveSpace":true,"text":"Hello"}}]}}],"author":"unnamed","date":"1970-01-01T00:00:00Z"}}],"property":{"runProperty":{},"tabs":[]},"hasNumbering":false}"#
        );
    }

    #[test]
    fn test_raw_text() {
        let b = Paragraph::new()
            .add_run(Run::new().add_text("Hello"))
            .add_insert(Insert::new(Run::new().add_text("World")))
            .add_delete(Delete::new().add_run(Run::new().add_delete_text("!!!!!")))
            .raw_text();
        assert_eq!(b, "HelloWorld".to_owned());
    }
}
