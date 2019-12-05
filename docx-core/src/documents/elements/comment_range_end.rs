use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone)]
pub struct CommentRangeEnd<'a> {
    id: &'a str,
}

impl<'a> CommentRangeEnd<'a> {
    pub fn new(id: &'a str) -> CommentRangeEnd<'a> {
        CommentRangeEnd { id }
    }
}

impl<'a> BuildXML for CommentRangeEnd<'a> {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.open_run()
            .open_run_property()
            .close()
            .close()
            .comment_range_end(&self.id)
            .open_run()
            .comment_reference(&self.id)
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
    fn test_comment_range_end() {
        let c = CommentRangeEnd::new("mockid");
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:r>
  <w:rPr />
</w:r>
<w:commentRangeEnd w:id="mockid" />
<w:r>
  <w:commentReference w:id="mockid" />
</w:r>"#
        );
    }
}
