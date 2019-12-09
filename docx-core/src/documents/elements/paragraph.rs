use super::*;
use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;

#[derive(Debug, Clone)]
pub struct Paragraph {
    pub children: Vec<ParagraphChild>,
    property: ParagraphProperty,
    attrs: Vec<(String, String)>,
}

impl Default for Paragraph {
    fn default() -> Self {
        Self {
            children: Vec::new(),
            property: ParagraphProperty::new(),
            attrs: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ParagraphChild {
    Run(Run),
    Insert(Insert),
    Delete(Delete),
    BookmarkStart(BookmarkStart),
    BookmarkEnd(BookmarkEnd),
    CommentStart(CommentRangeStart),
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

impl Paragraph {
    pub fn new() -> Paragraph {
        Default::default()
    }

    pub fn children(&self) -> &Vec<ParagraphChild> {
        &self.children
    }

    pub fn add_run(mut self, run: Run) -> Paragraph {
        self.children.push(ParagraphChild::Run(run));
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

    pub fn add_attr(mut self, key: impl Into<String>, val: impl Into<String>) -> Paragraph {
        self.attrs.push((key.into(), val.into()));
        self
    }

    pub fn add_bookmark_start(
        mut self,
        id: impl Into<String>,
        name: impl Into<String>,
    ) -> Paragraph {
        self.children
            .push(ParagraphChild::BookmarkStart(BookmarkStart::new(id, name)));
        self
    }

    pub fn add_bookmark_end(mut self, id: impl Into<String>) -> Paragraph {
        self.children
            .push(ParagraphChild::BookmarkEnd(BookmarkEnd::new(id)));
        self
    }

    pub fn add_comment_start(mut self, comment: Comment) -> Paragraph {
        self.children
            .push(ParagraphChild::CommentStart(CommentRangeStart::new(
                comment,
            )));
        self
    }

    pub fn add_comment_end(mut self, id: impl Into<String>) -> Paragraph {
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

    pub fn indent(mut self, left: usize, special_indent: Option<SpecialIndentType>) -> Paragraph {
        self.property = self.property.indent(left, special_indent);
        self
    }

    pub fn numbering(mut self, id: NumberingId, level: IndentLevel) -> Self {
        self.property = self.property.numbering(id, level);
        self
    }
}

impl BuildXML for Paragraph {
    fn build(&self) -> Vec<u8> {
        XMLBuilder::new()
            .open_paragraph(&self.attrs)
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
            r#"<w:p><w:pPr><w:pStyle w:val="Normal" /><w:rPr /></w:pPr><w:r><w:rPr /><w:t xml:space="preserve">Hello</w:t></w:r></w:p>"#
        );
    }

    #[test]
    fn test_custom_attr() {
        let b = Paragraph::new()
            .add_run(Run::new().add_text("Hello"))
            .add_attr("customId", "abcd-1234-567890")
            .build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:p customId="abcd-1234-567890"><w:pPr><w:pStyle w:val="Normal" /><w:rPr /></w:pPr><w:r><w:rPr /><w:t xml:space="preserve">Hello</w:t></w:r></w:p>"#
        );
    }

    #[test]
    fn test_bookmark() {
        let b = Paragraph::new()
            .add_bookmark_start("1234-5678", "article")
            .add_run(Run::new().add_text("Hello"))
            .add_bookmark_end("1234-5678")
            .build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:p><w:pPr><w:pStyle w:val="Normal" /><w:rPr /></w:pPr><w:bookmarkStart w:id="1234-5678" w:name="article" /><w:r><w:rPr /><w:t xml:space="preserve">Hello</w:t></w:r><w:bookmarkEnd w:id="1234-5678" /></w:p>"#
        );
    }

    #[test]
    fn test_comment() {
        let b = Paragraph::new()
            .add_comment_start(
                Comment::new("1234-5678"), // .paragraph(Paragraph::new().add_run(Run::new().add_text("Comment"))),
            )
            .add_run(Run::new().add_text("Hello"))
            .add_comment_end("1234-5678")
            .build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:p><w:pPr><w:pStyle w:val="Normal" /><w:rPr /></w:pPr><w:commentRangeStart w:id="1234-5678" /><w:r><w:rPr /><w:t xml:space="preserve">Hello</w:t></w:r><w:r>
  <w:rPr />
</w:r>
<w:commentRangeEnd w:id="1234-5678" />
<w:r>
  <w:commentReference w:id="1234-5678" />
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
            r#"<w:p><w:pPr><w:pStyle w:val="Normal" /><w:rPr /><w:numPr><w:numId w:val="0" /><w:ilvl w:val="1" /></w:numPr></w:pPr><w:r><w:rPr /><w:t xml:space="preserve">Hello</w:t></w:r></w:p>"#
        );
    }
}
