mod build_xml;
mod content_types;
mod doc_props;
mod elements;
mod rels;
mod styles;
mod xml_document;

pub(crate) use build_xml::*;

use crate::xml_builder::*;
use content_types::*;
use doc_props::*;
use elements::*;
use rels::*;
use styles::*;

pub(crate) struct Document {
    content_type: ContentTypes,
    rels: Rels,
    doc_props: DocProps,
}

impl Document {
    pub fn new() -> Document {
        let content_type = ContentTypes::new();
        let rels = Rels::new();
        let doc_props = DocProps::new(None, None /* TODO: */);
        Document {
            content_type,
            rels,
            doc_props,
        }
    }
}
