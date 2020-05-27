use serde::Serialize;

use crate::documents::{BuildXML, Paragraph};
use crate::xml_builder::*;

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct Comment {
    pub id: usize,
    pub author: String,
    pub date: String,
    pub paragraph: Paragraph,
}

impl Default for Comment {
    fn default() -> Comment {
        Comment {
            id: 1,
            author: "unnamed".to_owned(),
            date: "1970-01-01T00:00:00Z".to_owned(),
            paragraph: Paragraph::new(),
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

    pub fn paragraph(mut self, p: Paragraph) -> Comment {
        self.paragraph = p;
        self
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

impl BuildXML for Comment {
    fn build(&self) -> Vec<u8> {
        XMLBuilder::new()
            .open_comment(&format!("{}", self.id), &self.author, &self.date, "")
            .add_child(&self.paragraph)
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
        let b = Comment::new(1).build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:comment w:id="1" w:author="unnamed" w:date="1970-01-01T00:00:00Z" w:initials=""><w:p><w:pPr><w:rPr /></w:pPr></w:p></w:comment>"#
        );
    }
}
