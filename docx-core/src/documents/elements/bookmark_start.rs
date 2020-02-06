use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, PartialEq)]
pub struct BookmarkStart {
    id: usize,
    name: String,
}

impl BookmarkStart {
    pub fn new(id: usize, name: impl Into<String>) -> BookmarkStart {
        BookmarkStart {
            id,
            name: name.into(),
        }
    }
}

impl BuildXML for BookmarkStart {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.bookmark_start(&format!("{}", self.id), &self.name)
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
    fn test_bookmark_start() {
        let c = BookmarkStart::new(0, "mockname");
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:bookmarkStart w:id="0" w:name="mockname" />"#
        );
    }
}
