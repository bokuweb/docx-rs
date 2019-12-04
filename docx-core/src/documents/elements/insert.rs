use crate::documents::{BuildXML, HistoryId, Run};
use crate::xml_builder::*;

#[derive(Debug, Clone)]
pub struct Insert<'a> {
    author: &'a str,
    date: &'a str,
    run: Run<'a>,
}

impl<'a> Default for Insert<'a> {
    fn default() -> Insert<'a> {
        Insert {
            author: "unnamed",
            date: "1970-01-01T00:00:00Z",
            run: Run::new(),
        }
    }
}

impl<'a> Insert<'a> {
    pub fn new() -> Insert<'a> {
        Default::default()
    }

    pub fn run(mut self, run: Run<'a>) -> Insert<'a> {
        self.run = run;
        self
    }
}

impl<'a> HistoryId for Insert<'a> {}

impl<'a> BuildXML for Insert<'a> {
    fn build(&self) -> Vec<u8> {
        XMLBuilder::new()
            .open_insert(&self.generate(), self.author, self.date)
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
        let b = Insert::new().build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:ins w:id="123" w:author="unnamed" w:date="1970-01-01T00:00:00Z"><w:r><w:rPr /></w:r></w:ins>"#
        );
    }
}
