use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;
use std::io::Write;

use crate::documents::*;
use crate::xml_builder::*;

#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    pub id: usize,
    pub author: String,
    pub date: String,
    pub children: Vec<CommentChild>,
    pub parent_comment_id: Option<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CommentChild {
    Paragraph(Paragraph),
    Table(Table),
}

impl Serialize for CommentChild {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            CommentChild::Paragraph(ref p) => {
                let mut t = serializer.serialize_struct("Paragraph", 2)?;
                t.serialize_field("type", "paragraph")?;
                t.serialize_field("data", p)?;
                t.end()
            }
            CommentChild::Table(ref c) => {
                let mut t = serializer.serialize_struct("Table", 2)?;
                t.serialize_field("type", "table")?;
                t.serialize_field("data", c)?;
                t.end()
            }
        }
    }
}

impl Default for Comment {
    fn default() -> Comment {
        Comment {
            id: 1,
            author: "unnamed".to_owned(),
            date: "1970-01-01T00:00:00Z".to_owned(),
            children: vec![],
            parent_comment_id: None,
        }
    }
}

impl Comment {
    pub fn new(id: usize) -> Comment {
        Self {
            id,
            ..Default::default()
        }
    }

    pub fn author(mut self, author: impl Into<String>) -> Comment {
        self.author = author.into();
        self
    }

    pub fn date(mut self, date: impl Into<String>) -> Comment {
        self.date = date.into();
        self
    }

    pub fn add_paragraph(mut self, p: Paragraph) -> Self {
        self.children.push(CommentChild::Paragraph(p));
        self
    }

    pub fn add_table(mut self, t: Table) -> Self {
        self.children.push(CommentChild::Table(t));
        self
    }

    pub fn parent_comment_id(mut self, parent_comment_id: usize) -> Comment {
        self.parent_comment_id = Some(parent_comment_id);
        self
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

impl BuildXML for CommentChild {
    fn build_to<W: Write>(
        &self,
        stream: xml::writer::EventWriter<W>,
    ) -> xml::writer::Result<xml::writer::EventWriter<W>> {
        match self {
            CommentChild::Paragraph(v) => v.build_to(stream),
            CommentChild::Table(v) => v.build_to(stream),
        }
    }
}

impl BuildXML for Comment {
    fn build_to<W: Write>(
        &self,
        stream: xml::writer::EventWriter<W>,
    ) -> xml::writer::Result<xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_comment(&self.id.to_string(), &self.author, &self.date, "")?
            .add_children(&self.children)?
            .close()?
            .into_inner()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_comment_default() {
        let b = Comment::new(1).build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:comment w:id="1" w:author="unnamed" w:date="1970-01-01T00:00:00Z" w:initials="" />"#
        );
    }

    #[test]
    fn test_comment_with_default_paragraph() {
        let b = Comment::new(1).add_paragraph(Paragraph::new()).build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:comment w:id="1" w:author="unnamed" w:date="1970-01-01T00:00:00Z" w:initials=""><w:p w14:paraId="12345678"><w:pPr /></w:p></w:comment>"#
        );
    }
}
