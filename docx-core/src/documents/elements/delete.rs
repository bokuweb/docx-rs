use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;

use crate::xml_builder::*;
use crate::{documents::*, escape};

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct Delete {
    pub author: String,
    pub date: String,
    pub children: Vec<DeleteChild>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DeleteChild {
    Run(Run),
    CommentStart(Box<CommentRangeStart>),
    CommentEnd(CommentRangeEnd),
}

impl Serialize for DeleteChild {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            DeleteChild::Run(ref r) => {
                let mut t = serializer.serialize_struct("Run", 2)?;
                t.serialize_field("type", "run")?;
                t.serialize_field("data", r)?;
                t.end()
            }
            DeleteChild::CommentStart(ref r) => {
                let mut t = serializer.serialize_struct("CommentRangeStart", 2)?;
                t.serialize_field("type", "commentRangeStart")?;
                t.serialize_field("data", r)?;
                t.end()
            }
            DeleteChild::CommentEnd(ref r) => {
                let mut t = serializer.serialize_struct("CommentRangeEnd", 2)?;
                t.serialize_field("type", "commentRangeEnd")?;
                t.serialize_field("data", r)?;
                t.end()
            }
        }
    }
}

impl Default for Delete {
    fn default() -> Delete {
        Delete {
            author: "unnamed".to_owned(),
            date: "1970-01-01T00:00:00Z".to_owned(),
            children: vec![],
        }
    }
}

impl Delete {
    pub fn new() -> Delete {
        Self {
            children: vec![],
            ..Default::default()
        }
    }

    pub fn add_run(mut self, run: Run) -> Delete {
        self.children.push(DeleteChild::Run(run));
        self
    }

    pub fn add_comment_start(mut self, comment: Comment) -> Delete {
        self.children
            .push(DeleteChild::CommentStart(Box::new(CommentRangeStart::new(
                comment,
            ))));
        self
    }

    pub fn add_comment_end(mut self, id: usize) -> Delete {
        self.children
            .push(DeleteChild::CommentEnd(CommentRangeEnd::new(id)));
        self
    }

    pub fn author(mut self, author: impl Into<String>) -> Delete {
        self.author = escape::escape(&author.into());
        self
    }

    pub fn date(mut self, date: impl Into<String>) -> Delete {
        self.date = date.into();
        self
    }
}

impl HistoryId for Delete {}

impl BuildXML for Delete {
    #[allow(clippy::needless_borrow)]
    fn build(&self) -> Vec<u8> {
        let id = self.generate();
        let mut b = XMLBuilder::new().open_delete(&id, &self.author, &self.date);
        for c in &self.children {
            match c {
                DeleteChild::Run(t) => b = b.add_child(t),
                DeleteChild::CommentStart(c) => b = b.add_child(c),
                DeleteChild::CommentEnd(c) => b = b.add_child(c),
            }
        }
        b.close().build()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_delete_default() {
        let b = Delete::new().add_run(Run::new()).build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:del w:id="123" w:author="unnamed" w:date="1970-01-01T00:00:00Z"><w:r><w:rPr /></w:r></w:del>"#
        );
    }
}
