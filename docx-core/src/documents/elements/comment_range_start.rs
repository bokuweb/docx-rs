use super::Comment;
use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone)]
pub struct CommentRangeStart {
    id: String,
    comment: Comment,
}

impl CommentRangeStart {
    pub fn new(comment: Comment) -> CommentRangeStart {
        CommentRangeStart {
            id: comment.id(),
            comment,
        }
    }

    pub(crate) fn comment(&self) -> Comment {
        self.comment.clone()
    }
}

impl BuildXML for CommentRangeStart {
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
