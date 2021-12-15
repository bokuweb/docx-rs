use serde::Serialize;

use super::*;
use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Serialize, Debug, Clone, PartialEq, Default)]
pub struct Hyperlink {
    pub rid: Option<String>,
    pub anchor: Option<String>,
    pub history: bool,
    pub children: Vec<ParagraphChild>,
}

impl Hyperlink {
    pub fn new() -> Self {
        Hyperlink::default()
    }

    pub fn rid(mut self, rid: impl Into<String>) -> Self {
        self.rid = Some(rid.into());
        self
    }

    pub fn anchor(mut self, anchor: impl Into<String>) -> Self {
        self.anchor = Some(anchor.into());
        self
    }

    pub fn history(mut self) -> Self {
        self.history = true;
        self
    }

    pub fn add_run(mut self, run: Run) -> Self {
        self.children.push(ParagraphChild::Run(Box::new(run)));
        self
    }

    pub fn add_structured_data_tag(mut self, t: StructuredDataTag) -> Self {
        self.children
            .push(ParagraphChild::StructuredDataTag(Box::new(t)));
        self
    }

    pub fn add_insert(mut self, insert: Insert) -> Self {
        self.children.push(ParagraphChild::Insert(insert));
        self
    }

    pub fn add_delete(mut self, delete: Delete) -> Self {
        self.children.push(ParagraphChild::Delete(delete));
        self
    }

    pub fn add_bookmark_start(mut self, id: usize, name: impl Into<String>) -> Self {
        self.children
            .push(ParagraphChild::BookmarkStart(BookmarkStart::new(id, name)));
        self
    }

    pub fn add_bookmark_end(mut self, id: usize) -> Self {
        self.children
            .push(ParagraphChild::BookmarkEnd(BookmarkEnd::new(id)));
        self
    }

    pub fn add_comment_start(mut self, comment: Comment) -> Self {
        self.children.push(ParagraphChild::CommentStart(Box::new(
            CommentRangeStart::new(comment),
        )));
        self
    }

    pub fn add_comment_end(mut self, id: usize) -> Self {
        self.children
            .push(ParagraphChild::CommentEnd(CommentRangeEnd::new(id)));
        self
    }
}

impl BuildXML for Hyperlink {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.open_hyperlink(self.rid.as_ref(), self.anchor.as_ref(), self.history)
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
    fn test_hyperlink() {
        let l = Hyperlink::new()
            .anchor("ToC1")
            .add_run(Run::new().add_text("hello"));
        let b = l.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:hyperlink w:anchor="ToC1"><w:r><w:rPr /><w:t xml:space="preserve">hello</w:t></w:r></w:hyperlink>"#
        );
    }
}
