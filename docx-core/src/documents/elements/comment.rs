use crate::documents::{BuildXML, Paragraph};
use crate::xml_builder::*;

#[derive(Debug, Clone)]
pub struct Comment {
    id: String,
    author: String,
    date: String,
    paragraph: Paragraph,
}

impl Default for Comment {
    fn default() -> Comment {
        Comment {
            id: "invalidId".to_owned(),
            author: "unnamed".to_owned(),
            date: "1970-01-01T00:00:00Z".to_owned(),
            paragraph: Paragraph::new(),
        }
    }
}

impl Comment {
    pub fn new(id: impl Into<String>) -> Comment {
        Self {
            id: id.into(),
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

    pub fn id(&self) -> String {
        self.id.clone()
    }
}

impl BuildXML for Comment {
    fn build(&self) -> Vec<u8> {
        XMLBuilder::new()
            .open_comment(&self.id, &self.author, &self.date, "")
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
        let b = Comment::new("123").build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:comment w:id="123" w:author="unnamed" w:date="1970-01-01T00:00:00Z" w:initials=""><w:p><w:pPr><w:pStyle w:val="Normal" /><w:rPr /></w:pPr></w:p></w:comment>"#
        );
    }
}
