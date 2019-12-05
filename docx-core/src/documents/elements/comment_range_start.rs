use super::Comment;
use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone)]
pub struct CommentRangeStart<'a> {
    id: &'a str,
    comment: Comment<'a>,
}

impl<'a> CommentRangeStart<'a> {
    pub fn new(comment: Comment<'a>) -> CommentRangeStart<'a> {
        CommentRangeStart {
            id: comment.id(),
            comment,
        }
    }

    pub(crate) fn comment(&self) -> Comment<'a> {
        self.comment.clone()
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
        let c = CommentRangeStart::new(Comment::new("mockid"));
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:commentRangeStart w:id="mockid" />"#
        );
    }
}
