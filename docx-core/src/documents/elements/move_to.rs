use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;
use std::io::Write;

use crate::xml_builder::*;
use crate::{documents::*, escape};

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct MoveTo {
    pub author: String,
    pub date: String,
    pub children: Vec<MoveToChild>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MoveToChild {
    Run(Run),
    CommentStart(Box<CommentRangeStart>),
    CommentEnd(CommentRangeEnd),
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

impl Default for MoveTo {
    fn default() -> Self {
        Self {
            author: "unnamed".to_owned(),
            date: "1970-01-01T00:00:00Z".to_owned(),
            children: vec![],
        }
    }
}

impl MoveTo {
    pub fn new() -> Self {
        Self {
            children: vec![],
            ..Default::default()
        }
    }

    pub fn add_run(mut self, run: Run) -> Self {
        self.children.push(MoveToChild::Run(run));
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

    pub fn author(mut self, author: impl Into<String>) -> Self {
        self.author = escape::escape(&author.into());
        self
    }

    pub fn date(mut self, date: impl Into<String>) -> Self {
        self.date = date.into();
        self
    }
}

impl HistoryId for MoveTo {}

impl BuildXML for MoveTo {
    fn build_to<W: Write>(
        &self,
        stream: xml::writer::EventWriter<W>,
    ) -> xml::writer::Result<xml::writer::EventWriter<W>> {
        let id = self.generate();
        XMLBuilder::from(stream)
            .open_move_to(&id, &self.author, &self.date)?
            .apply_each(&self.children, |ch, b| match ch {
                MoveToChild::Run(t) => b.add_child(t),
                MoveToChild::CommentStart(c) => b.add_child(&c),
                MoveToChild::CommentEnd(c) => b.add_child(c),
            })?
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
        let b = MoveTo::new().add_run(Run::new()).build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:moveTo w:id="123" w:author="unnamed" w:date="1970-01-01T00:00:00Z"><w:r><w:rPr /></w:r></w:moveTo>"#
        );
    }
}
