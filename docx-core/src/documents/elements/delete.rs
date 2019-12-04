use crate::documents::{BuildXML, HistoryId, Run};
use crate::xml_builder::*;

#[derive(Debug, Clone)]
pub struct Delete<'a> {
    author: &'a str,
    date: &'a str,
    run: Run<'a>,
}

impl<'a> Default for Delete<'a> {
    fn default() -> Delete<'a> {
        Delete {
            author: "unnamed",
            date: "1970-01-01T00:00:00Z",
            run: Run::new(),
        }
    }
}

impl<'a> Delete<'a> {
    pub fn new() -> Delete<'a> {
        Default::default()
    }

    pub fn run(mut self, run: Run<'a>) -> Delete<'a> {
        self.run = run;
        self
    }
}

impl<'a> HistoryId for Delete<'a> {}

impl<'a> BuildXML for Delete<'a> {
    fn build(&self) -> Vec<u8> {
        XMLBuilder::new()
            .open_delete(&self.generate(), self.author, self.date)
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
