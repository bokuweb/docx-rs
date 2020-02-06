use crate::documents::{BuildXML, HistoryId, Run};
use crate::xml_builder::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Insert {
    pub run: Run,
    pub author: String,
    pub date: String,
}

impl Default for Insert {
    fn default() -> Insert {
        Insert {
            author: "unnamed".to_owned(),
            date: "1970-01-01T00:00:00Z".to_owned(),
            run: Run::new(),
        }
    }
}

impl Insert {
    pub fn new(run: Run) -> Insert {
        Self {
            run,
            ..Default::default()
        }
    }

    pub fn author(mut self, author: impl Into<String>) -> Insert {
        self.author = author.into();
        self
    }

    pub fn date(mut self, date: impl Into<String>) -> Insert {
        self.date = date.into();
        self
    }
}

impl HistoryId for Insert {}

impl BuildXML for Insert {
    fn build(&self) -> Vec<u8> {
        XMLBuilder::new()
            .open_insert(&self.generate(), &self.author, &self.date)
            .add_child(&self.run)
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
