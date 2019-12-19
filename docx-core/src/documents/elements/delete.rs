use crate::documents::{BuildXML, HistoryId, Run};
use crate::xml_builder::*;

#[derive(Debug, Clone)]
pub struct Delete {
    pub author: String,
    pub date: String,
    pub run: Run,
}

impl Default for Delete {
    fn default() -> Delete {
        Delete {
            author: "unnamed".to_owned(),
            date: "1970-01-01T00:00:00Z".to_owned(),
            run: Run::new(),
        }
    }
}

impl Delete {
    pub fn new() -> Delete {
        Default::default()
    }

    pub fn run(mut self, run: Run) -> Delete {
        self.run = run;
        self
    }

    pub fn author(mut self, author: impl Into<String>) -> Delete {
        self.author = author.into();
        self
    }

    pub fn date(mut self, date: impl Into<String>) -> Delete {
        self.date = date.into();
        self
    }
}

impl HistoryId for Delete {}

impl BuildXML for Delete {
    fn build(&self) -> Vec<u8> {
        XMLBuilder::new()
            .open_delete(&self.generate(), &self.author, &self.date)
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
    fn test_delete_default() {
        let b = Delete::new().build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:del w:id="123" w:author="unnamed" w:date="1970-01-01T00:00:00Z"><w:r><w:rPr /></w:r></w:del>"#
        );
    }
}
