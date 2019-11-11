mod build_xml;
mod content_types;
mod doc_props;
mod document;
mod elements;
mod rels;
mod styles;
mod xml_docx;

pub(crate) use build_xml::*;

pub use content_types::*;
pub use doc_props::*;
pub use document::*;
pub use elements::*;
pub use rels::*;
pub use styles::*;
pub use xml_docx::*;

#[derive(Debug)]
pub struct Docx {
    content_type: ContentTypes,
    rels: Rels,
    doc_props: DocProps,
    styles: Styles,
    document: Document,
}

impl Default for Docx {
    fn default() -> Self {
        let content_type = ContentTypes::new();
        let rels = Rels::new();
        let doc_props = DocProps::new(None, None /* TODO: */);
        let styles = Styles::new();
        let document = Document::new();
        Docx {
            content_type,
            rels,
            doc_props,
            styles,
            document,
        }
    }
}

impl Docx {
    pub fn new() -> Docx {
        Default::default()
    }

    pub fn add_paragraph(mut self, p: Paragraph) -> Docx {
        self.document = self.document.add_paragraph(p);
        self
    }

    pub fn build(&self) -> XMLDocx {
        XMLDocx {
            content_type: self.content_type.build(),
            rels: self.rels.build(),
            doc_props: self.doc_props.build(),
            styles: self.styles.build(),
            document: self.document.build(),
        }
    }
}
