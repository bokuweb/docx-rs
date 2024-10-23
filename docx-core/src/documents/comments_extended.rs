use serde::Serialize;
use std::io::Write;

use super::*;
use crate::documents::BuildXML;
use crate::xml_builder::*;

// i.e.    <w15:commentEx w15:paraId="00000001" w15:paraIdParent="57D1BD7C" w15:done="0"/>
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CommentsExtended {
    pub children: Vec<CommentExtended>,
}

impl CommentsExtended {
    pub fn new() -> CommentsExtended {
        Default::default()
    }

    pub fn add_comments_extended(&mut self, c: Vec<CommentExtended>) {
        self.children = c;
    }
}

impl BuildXML for CommentsExtended {
    fn build_to<W: Write>(
        &self,
        stream: xml::writer::EventWriter<W>,
    ) -> xml::writer::Result<xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_comments_extended()?
            .add_children(&self.children)?
            .close()?
            .into_inner()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use insta::assert_snapshot;
    use std::str;

    #[test]
    fn test_settings() {
        let mut c = CommentsExtended::new();
        c.add_comments_extended(vec![CommentExtended::new("123")]);
        let b = c.build();
        assert_snapshot!("comments_extended_snapshot", str::from_utf8(&b).unwrap());
    }
}
