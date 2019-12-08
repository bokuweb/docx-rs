use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone)]
pub struct BookmarkEnd {
    id: String,
}

impl BookmarkEnd {
    pub fn new(id: impl Into<String>) -> BookmarkEnd {
        BookmarkEnd { id: id.into() }
    }
}

impl BuildXML for BookmarkEnd {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.bookmark_end(&self.id).build()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_bookmark_end() {
        let c = BookmarkEnd::new("mockid");
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:bookmarkEnd w:id="mockid" />"#
        );
    }
}
