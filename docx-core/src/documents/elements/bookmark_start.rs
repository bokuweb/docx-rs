use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, PartialEq)]
pub struct BookmarkStart {
    id: String,
    name: String,
}

impl BookmarkStart {
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> BookmarkStart {
        BookmarkStart {
            id: id.into(),
            name: name.into(),
        }
    }
}

impl BuildXML for BookmarkStart {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.bookmark_start(&self.id, &self.name).build()
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
        let c = BookmarkStart::new("mockid", "mockname");
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:bookmarkStart w:id="mockid" w:name="mockname" />"#
        );
    }
}
