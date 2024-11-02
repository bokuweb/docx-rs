use serde::Serialize;
use std::io::Write;

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct CommentRangeEnd {
    id: usize,
}

impl CommentRangeEnd {
    pub fn new(id: usize) -> CommentRangeEnd {
        CommentRangeEnd { id }
    }
}

impl BuildXML for CommentRangeEnd {
    fn build_to<W: Write>(
        &self,
        stream: xml::writer::EventWriter<W>,
    ) -> xml::writer::Result<xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_run()?
            .open_run_property()?
            .close()?
            .close()?
            .comment_range_end(&format!("{}", self.id))?
            .open_run()?
            .comment_reference(&format!("{}", self.id))?
            .close()?
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
    fn test_comment_range_end() {
        let c = CommentRangeEnd::new(1);
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:r><w:rPr /></w:r><w:commentRangeEnd w:id="1" /><w:r><w:commentReference w:id="1" /></w:r>"#
        );
    }
}
