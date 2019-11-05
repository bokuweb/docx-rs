mod content_types;
mod doc_props;
mod rels;
mod xml_document;

use content_types::*;
use doc_props::*;
use rels::*;

pub(crate) struct Document {
    content_type: ContentTypes,
    rels: Rels,
    doc_props: DocProps,
}

impl Document {
    pub fn new() -> Document {
        let content_type = ContentTypes::new();
        let rels = Rels::new();
        let doc_props = DocProps::new(None /* TODO: */);
        Document {
            content_type,
            rels,
            doc_props,
        }
    }
}
