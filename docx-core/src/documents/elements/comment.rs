use crate::documents::{BuildXML, Paragraph};
use crate::xml_builder::*;

#[derive(Debug, Clone)]
pub struct Comment<'a> {
    id: &'a str,
    author: &'a str,
    date: &'a str,
    paragraph: Paragraph<'a>,
}

impl<'a> Default for Comment<'a> {
    fn default() -> Comment<'a> {
        Comment {
            id: "invalidId",
            author: "unnamed",
            date: "1970-01-01T00:00:00Z",
            paragraph: Paragraph::new(),
        }
    }
}

impl<'a> Comment<'a> {
    pub fn new(id: &'a str) -> Comment<'a> {
        Self {
            id,
            ..Default::default()
        }
    }

    pub fn author(mut self, author: &'a str) -> Comment<'a> {
        self.author = author;
        self
    }

    pub fn date(mut self, date: &'a str) -> Comment<'a> {
        self.date = date;
        self
    }

    pub fn paragraph(mut self, p: Paragraph<'a>) -> Comment<'a> {
        self.paragraph = p;
        self
    }

    pub fn id(&self) -> &'a str {
        self.id
    }
}

impl<'a> BuildXML for Comment<'a> {
    fn build(&self) -> Vec<u8> {
        XMLBuilder::new()
            .open_comment(&self.id, self.author, self.date, "")
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
