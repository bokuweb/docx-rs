pub mod content_types;
mod xml_builder;
mod xml_document;

use content_types::*;

pub(crate) struct Document {
    content_type: ContentTypes,
}

impl Document {
    pub fn new() -> Document {
        let content_type = ContentTypes::new();
        Document { content_type }
    }
}
