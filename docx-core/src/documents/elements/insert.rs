use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;

use super::*;

use crate::documents::{BuildXML, HistoryId, Run};
use crate::{escape, xml_builder::*};

#[derive(Debug, Clone, PartialEq)]
pub enum InsertChild {
    Run(Box<Run>),
    Delete(Delete),
    CommentStart(Box<CommentRangeStart>),
    CommentEnd(CommentRangeEnd),
}

impl BuildXML for InsertChild {
    fn build(&self) -> Vec<u8> {
        match self {
            InsertChild::Run(v) => v.build(),
            InsertChild::Delete(v) => v.build(),
            InsertChild::CommentStart(v) => v.build(),
            InsertChild::CommentEnd(v) => v.build(),
        }
    }
}

impl Serialize for InsertChild {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            InsertChild::Run(ref r) => {
                let mut t = serializer.serialize_struct("Run", 2)?;
                t.serialize_field("type", "run")?;
                t.serialize_field("data", r)?;
                t.end()
            }
            InsertChild::Delete(ref r) => {
                let mut t = serializer.serialize_struct("Delete", 2)?;
                t.serialize_field("type", "delete")?;
                t.serialize_field("data", r)?;
                t.end()
            }
            InsertChild::CommentStart(ref r) => {
                let mut t = serializer.serialize_struct("CommentRangeStart", 2)?;
                t.serialize_field("type", "commentRangeStart")?;
                t.serialize_field("data", r)?;
                t.end()
            }
            InsertChild::CommentEnd(ref r) => {
                let mut t = serializer.serialize_struct("CommentRangeEnd", 2)?;
                t.serialize_field("type", "commentRangeEnd")?;
                t.serialize_field("data", r)?;
                t.end()
            }
        }
    }
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct Insert {
    pub children: Vec<InsertChild>,
    pub author: String,
    pub date: String,
}

impl Default for Insert {
    fn default() -> Insert {
        Insert {
            author: "unnamed".to_owned(),
            date: "1970-01-01T00:00:00Z".to_owned(),
            children: vec![],
        }
    }
}

impl Insert {
    pub fn new(run: Run) -> Insert {
        Self {
            children: vec![InsertChild::Run(Box::new(run))],
            ..Default::default()
        }
    }

    pub fn new_with_empty() -> Insert {
        Self {
            ..Default::default()
        }
    }

    pub fn new_with_del(del: Delete) -> Insert {
        Self {
            children: vec![InsertChild::Delete(del)],
            ..Default::default()
        }
    }

    pub fn add_run(mut self, run: Run) -> Insert {
        self.children.push(InsertChild::Run(Box::new(run)));
        self
    }

    pub fn add_delete(mut self, del: Delete) -> Insert {
        self.children.push(InsertChild::Delete(del));
        self
    }

    pub fn add_child(mut self, c: InsertChild) -> Insert {
        self.children.push(c);
        self
    }

    pub fn add_comment_start(mut self, comment: Comment) -> Self {
        self.children
            .push(InsertChild::CommentStart(Box::new(CommentRangeStart::new(
                comment,
            ))));
        self
    }

    pub fn add_comment_end(mut self, id: usize) -> Self {
        self.children
            .push(InsertChild::CommentEnd(CommentRangeEnd::new(id)));
        self
    }

    pub fn author(mut self, author: impl Into<String>) -> Insert {
        self.author = escape::escape(&author.into());
        self
    }

    pub fn date(mut self, date: impl Into<String>) -> Insert {
        self.date = date.into();
        self
    }
}

impl HistoryId for Insert {}

impl BuildXML for Insert {
    #[allow(clippy::needless_borrow)]
    fn build(&self) -> Vec<u8> {
        XMLBuilder::new()
            .open_insert(&self.generate(), &self.author, &self.date)
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
    fn test_ins_default() {
        let b = Insert::new(Run::new()).build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:ins w:id="123" w:author="unnamed" w:date="1970-01-01T00:00:00Z"><w:r><w:rPr /></w:r></w:ins>"#
        );
    }
}
