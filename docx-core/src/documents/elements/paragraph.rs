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
    BookmarkEnd(BookmarkEnd),
    CommentStart(Box<CommentRangeStart>),
    CommentEnd(CommentRangeEnd),
}

impl BuildXML for ParagraphChild {
    fn build(&self) -> Vec<u8> {
        match self {
            ParagraphChild::Run(v) => v.build(),
            ParagraphChild::Insert(v) => v.build(),
            ParagraphChild::Delete(v) => v.build(),
            ParagraphChild::BookmarkStart(v) => v.build(),
            ParagraphChild::BookmarkEnd(v) => v.build(),
            ParagraphChild::CommentStart(v) => v.build(),
            ParagraphChild::CommentEnd(v) => v.build(),
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

    pub fn keep_next(mut self, v: bool) -> Self {
        self.property = self.property.keep_next(v);
        self
    }

    pub fn keep_lines(mut self, v: bool) -> Self {
        self.property = self.property.keep_lines(v);
        self
    }

    pub fn page_break_before(mut self, v: bool) -> Self {
        self.property = self.property.page_break_before(v);
        self
    }

    pub fn window_control(mut self, v: bool) -> Self {
        self.property = self.property.window_control(v);
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

    pub(crate) fn hanging_chars(mut self, chars: i32) -> Paragraph {
        self.property = self.property.hanging_chars(chars);
        self
    }

    pub(crate) fn first_line_chars(mut self, chars: i32) -> Paragraph {
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

    pub(crate) fn run_property(mut self, p: RunProperty) -> Self {
        self.property.run_property = p;
        self
    }

    pub fn line_height(mut self, h: u32) -> Self {
        self.property = self.property.line_height(h);
        self
    }
}

impl BuildXML for Paragraph {
    fn build(&self) -> Vec<u8> {
        XMLBuilder::new()
            .open_paragraph(&self.id)
            .add_child(&self.property)
            .add_children(&self.children)
            .close()
            .build()
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
    fn test_paragraph_run_json() {
        let run = Run::new().add_text("Hello");
        let p = Paragraph::new().add_run(run);
        assert_eq!(
            serde_json::to_string(&p).unwrap(),
            r#"{"id":"12345678","children":[{"type":"run","data":{"runProperty":{"sz":null,"szCs":null,"color":null,"highlight":null,"underline":null,"bold":null,"boldCs":null,"italic":null,"italicCs":null,"vanish":null,"spacing":null,"fonts":null,"textBorder":null},"children":[{"type":"text","data":{"preserveSpace":true,"text":"Hello"}}]}}],"property":{"runProperty":{"sz":null,"szCs":null,"color":null,"highlight":null,"underline":null,"bold":null,"boldCs":null,"italic":null,"italicCs":null,"vanish":null,"spacing":null,"fonts":null,"textBorder":null},"style":null,"numberingProperty":null,"alignment":null,"indent":null,"lineHeight":null,"keepNext":false,"keepLines":false,"pageBreakBefore":false,"windowControl":false,"divId":null},"hasNumbering":false}"#
        );
    }

    #[test]
    fn test_paragraph_insert_json() {
        let run = Run::new().add_text("Hello");
        let ins = Insert::new(run);
        let p = Paragraph::new().add_insert(ins);
        assert_eq!(
            serde_json::to_string(&p).unwrap(),
            r#"{"id":"12345678","children":[{"type":"insert","data":{"children":[{"type":"run","data":{"runProperty":{"sz":null,"szCs":null,"color":null,"highlight":null,"underline":null,"bold":null,"boldCs":null,"italic":null,"italicCs":null,"vanish":null,"spacing":null,"fonts":null,"textBorder":null},"children":[{"type":"text","data":{"preserveSpace":true,"text":"Hello"}}]}}],"author":"unnamed","date":"1970-01-01T00:00:00Z"}}],"property":{"runProperty":{"sz":null,"szCs":null,"color":null,"highlight":null,"underline":null,"bold":null,"boldCs":null,"italic":null,"italicCs":null,"vanish":null,"spacing":null,"fonts":null,"textBorder":null},"style":null,"numberingProperty":null,"alignment":null,"indent":null,"lineHeight":null,"keepNext":false,"keepLines":false,"pageBreakBefore":false,"windowControl":false,"divId":null},"hasNumbering":false}"#
        );
    }
}
