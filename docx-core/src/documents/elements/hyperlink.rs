use serde::Serialize;

use super::*;
use crate::documents::BuildXML;
use crate::escape::escape;
use crate::types::*;
use crate::{create_hyperlink_rid, generate_hyperlink_id, xml_builder::*};

#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum HyperlinkData {
    External {
        rid: String,
        // path is writer only
        #[serde(skip_serializing_if = "String::is_empty")]
        path: String,
    },
    Anchor {
        anchor: String,
    },
}

#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Hyperlink {
    #[serde(flatten)]
    pub link: HyperlinkData,
    pub history: Option<usize>,
    pub children: Vec<ParagraphChild>,
}

impl Hyperlink {
    pub fn new(value: impl Into<String>, t: HyperlinkType) -> Self {
        let link = {
            match t {
                HyperlinkType::External => HyperlinkData::External {
                    rid: create_hyperlink_rid(generate_hyperlink_id()),
                    path: escape(&value.into()),
                },
                HyperlinkType::Anchor => HyperlinkData::Anchor {
                    anchor: value.into(),
                },
            }
        };
        Hyperlink {
            link,
            history: None,
            children: vec![],
        }
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
        let mut b = XMLBuilder::new();
        match self.link {
            HyperlinkData::Anchor { ref anchor } => {
                b = b.open_hyperlink(
                    None,
                    Some(anchor.clone()).as_ref(),
                    Some(self.history.unwrap_or(1)),
                )
            }
            HyperlinkData::External { ref rid, .. } => {
                b = b.open_hyperlink(
                    Some(rid.clone()).as_ref(),
                    None,
                    Some(self.history.unwrap_or(1)),
                )
            }
        };
        b.add_children(&self.children).close().build()
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
        let l = Hyperlink::new("ToC1", HyperlinkType::Anchor).add_run(Run::new().add_text("hello"));
        let b = l.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:hyperlink w:anchor="ToC1" w:history="1"><w:r><w:rPr /><w:t xml:space="preserve">hello</w:t></w:r></w:hyperlink>"#
        );
    }
}
