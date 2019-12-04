use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone)]
pub struct CommentRangeStart<'a> {
    id: &'a str,
}

impl<'a> CommentRangeStart<'a> {
    pub fn new(id: &'a str) -> CommentRangeStart<'a> {
        CommentRangeStart { id }
    }
}

impl<'a> BuildXML for CommentRangeStart<'a> {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.comment_range_start(&self.id).build()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_comment_range_start() {
        let c = CommentRangeStart::new("mockid");
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:commentRangeStart w:id="mockid" />"#
        );
    }
}
