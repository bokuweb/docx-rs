use serde::Serialize;

use super::Comment;
use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct CommentRangeStart {
    pub id: usize,
    pub comment: Comment,
}

impl CommentRangeStart {
    pub fn new(comment: Comment) -> CommentRangeStart {
        CommentRangeStart {
            id: comment.id(),
            comment,
        }
    }

    pub(crate) fn comment(&mut self, comment: Comment) {
        self.comment = comment;
    }

    pub(crate) fn get_comment(&self) -> Comment {
        self.comment.clone()
    }

    pub(crate) fn get_id(&self) -> usize {
        self.id
    }
}

impl BuildXML for CommentRangeStart {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.comment_range_start(&format!("{}", self.id)).build()
    }
}

impl BuildXML for Box<CommentRangeStart> {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.comment_range_start(&format!("{}", self.id)).build()
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
        let c = CommentRangeStart::new(Comment::new(1));
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:commentRangeStart w:id="1" />"#
        );
    }
}
