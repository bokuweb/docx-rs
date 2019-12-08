use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone)]
pub struct CommentRangeEnd {
    id: String,
}

impl CommentRangeEnd {
    pub fn new(id: impl Into<String>) -> CommentRangeEnd {
        CommentRangeEnd { id: id.into() }
    }
}

impl BuildXML for CommentRangeEnd {
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
