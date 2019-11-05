mod content_types;
mod rels;
mod xml_document;

use content_types::*;
use rels::*;

pub(crate) struct Document {
    content_type: ContentTypes,
    rels: Rels,
}

impl Document {
    pub fn new() -> Document {
        let content_type = ContentTypes::new();
        let rels = Rels::new();
        Document { content_type, rels }
    }
}
