use serde::Serialize;
use std::io::Write;

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct BookmarkEnd {
    pub id: usize,
}

impl BookmarkEnd {
    pub fn new(id: usize) -> BookmarkEnd {
        BookmarkEnd { id }
    }
}

impl BuildXML for BookmarkEnd {
    fn build_to<W: Write>(
        &self,
        stream: xml::writer::EventWriter<W>,
    ) -> xml::writer::Result<xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .bookmark_end(&self.id.to_string())?
            .into_inner()
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
        let c = BookmarkEnd::new(0);
        let b = c.build();
        assert_eq!(str::from_utf8(&b).unwrap(), r#"<w:bookmarkEnd w:id="0" />"#);
    }
}
