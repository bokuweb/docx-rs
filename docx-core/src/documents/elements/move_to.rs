use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;
use std::io::Write;

use super::*;

use crate::documents::{BuildXML, HistoryId, Run};
use crate::{escape, xml_builder::*};

#[derive(Debug, Clone, PartialEq)]
pub enum MoveToChild {
    Run(Box<Run>),
    Delete(Delete),
    CommentStart(Box<CommentRangeStart>),
    CommentEnd(CommentRangeEnd),
}

impl BuildXML for MoveToChild {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        match self {
            MoveToChild::Run(v) => v.build_to(stream),
            MoveToChild::Delete(v) => v.build_to(stream),
            MoveToChild::CommentStart(v) => v.build_to(stream),
            MoveToChild::CommentEnd(v) => v.build_to(stream),
        }
    }
}

impl Serialize for MoveToChild {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            MoveToChild::Run(ref r) => {
                let mut t = serializer.serialize_struct("Run", 2)?;
                t.serialize_field("type", "run")?;
                t.serialize_field("data", r)?;
                t.end()
            }
            MoveToChild::Delete(ref r) => {
                let mut t = serializer.serialize_struct("Delete", 2)?;
                t.serialize_field("type", "delete")?;
                t.serialize_field("data", r)?;
                t.end()
            }
            MoveToChild::CommentStart(ref r) => {
                let mut t = serializer.serialize_struct("CommentRangeStart", 2)?;
                t.serialize_field("type", "commentRangeStart")?;
                t.serialize_field("data", r)?;
                t.end()
            }
            MoveToChild::CommentEnd(ref r) => {
                let mut t = serializer.serialize_struct("CommentRangeEnd", 2)?;
                t.serialize_field("type", "commentRangeEnd")?;
                t.serialize_field("data", r)?;
                t.end()
            }
        }
    }
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct MoveTo {
    pub children: Vec<MoveToChild>,
    pub author: String,
    pub date: String,
}

impl Default for MoveTo {
    fn default() -> MoveTo {
        MoveTo {
            author: "unnamed".to_owned(),
            date: "1970-01-01T00:00:00Z".to_owned(),
            children: vec![],
        }
    }
}

impl MoveTo {
    pub fn new(run: Run) -> MoveTo {
        Self {
            children: vec![MoveToChild::Run(Box::new(run))],
            ..Default::default()
        }
    }

    pub fn new_with_empty() -> MoveTo {
        Self {
            ..Default::default()
        }
    }

    pub fn add_run(mut self, run: Run) -> MoveTo {
        self.children.push(MoveToChild::Run(Box::new(run)));
        self
    }

    pub fn add_delete(mut self, del: Delete) -> MoveTo {
        self.children.push(MoveToChild::Delete(del));
        self
    }

    pub fn add_child(mut self, c: MoveToChild) -> MoveTo {
        self.children.push(c);
        self
    }

    pub fn add_comment_start(mut self, comment: Comment) -> Self {
        self.children
            .push(MoveToChild::CommentStart(Box::new(CommentRangeStart::new(
                comment,
            ))));
        self
    }

    pub fn add_comment_end(mut self, id: usize) -> Self {
        self.children
            .push(MoveToChild::CommentEnd(CommentRangeEnd::new(id)));
        self
    }

    pub fn author(mut self, author: impl Into<String>) -> MoveTo {
        self.author = escape::escape(&author.into());
        self
    }

    pub fn date(mut self, date: impl Into<String>) -> MoveTo {
        self.date = date.into();
        self
    }
}

impl HistoryId for MoveTo {}

impl BuildXML for MoveTo {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_move_to(&self.generate(), &self.author, &self.date)?
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
    fn test_move_to_default() {
        let b = MoveTo::new(Run::new()).build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:moveTo w:id="123" w:author="unnamed" w:date="1970-01-01T00:00:00Z"><w:r><w:rPr /></w:r></w:moveTo>"#
        );
    }
}
