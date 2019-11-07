mod build_xml;
mod content_types;
mod doc_props;
mod document;
mod elements;
mod rels;
mod styles;
mod xml_document;

pub(crate) use build_xml::*;

pub use crate::xml_builder::*;
pub use content_types::*;
pub use doc_props::*;
pub use document::*;
pub use elements::*;
pub use rels::*;
pub use styles::*;

pub(crate) struct Docx {
    content_type: ContentTypes,
    rels: Rels,
    doc_props: DocProps,
}

impl Docx {
    pub fn new() -> Docx {
        let content_type = ContentTypes::new();
        let rels = Rels::new();
        let doc_props = DocProps::new(None, None /* TODO: */);
        Docx {
            content_type,
            rels,
            doc_props,
        }
    }
}
