use serde::Serialize;

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentExtended {
    pub paragraph_id: String,
    pub done: bool,
    pub parent_paragraph_id: Option<String>,
}

impl CommentExtended {
    pub fn new(paragraph_id: impl Into<String>) -> CommentExtended {
        Self {
            paragraph_id: paragraph_id.into(),
            done: false,
            parent_paragraph_id: None,
        }
    }

    pub fn done(mut self) -> CommentExtended {
        self.done = true;
        self
    }

    pub fn parent_paragraph_id(mut self, id: impl Into<String>) -> CommentExtended {
        self.parent_paragraph_id = Some(id.into());
        self
    }
}

impl BuildXML for CommentExtended {
    fn build(&self) -> Vec<u8> {
        XMLBuilder::new()
            .comment_extended(&self.paragraph_id, self.done, &self.parent_paragraph_id)
            .build()
    }
}
