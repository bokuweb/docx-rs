use serde::Serialize;

use super::*;
use crate::documents::BuildXML;
use crate::xml_builder::*;

// i.e.    <w15:commentEx w15:paraId="00000001" w15:paraIdParent="57D1BD7C" w15:done="0"/>
#[derive(Debug, Clone, PartialEq, Serialize)]
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

impl Default for CommentsExtended {
    fn default() -> Self {
        Self { children: vec![] }
    }
}

impl BuildXML for CommentsExtended {
    fn build(&self) -> Vec<u8> {
        let mut b = XMLBuilder::new();
        b = b.open_comments_extended();

        for c in &self.children {
            b = b.add_child(c)
        }
        b.close().build()
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
