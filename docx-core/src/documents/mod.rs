mod build_xml;
mod comments;
mod content_types;
mod doc_props;
mod document;
mod document_rels;
mod elements;
mod font_table;
mod history_id;
mod rels;
mod settings;
mod styles;
mod xml_docx;

pub(crate) use build_xml::BuildXML;
pub(crate) use history_id::HistoryId;

pub use comments::*;
pub use content_types::*;
pub use doc_props::*;
pub use document::*;
pub use document_rels::*;
pub use elements::*;
pub use font_table::*;
pub use rels::*;
pub use settings::*;
pub use styles::*;
pub use xml_docx::*;

#[derive(Debug)]
pub struct Docx<'a> {
    content_type: ContentTypes,
    rels: Rels,
    document_rels: DocumentRels,
    doc_props: DocProps<'a>,
    styles: Styles,
    document: Document<'a>,
    comments: Comments,
    settings: Settings,
    font_table: FontTable,
}

impl<'a> Default for Docx<'a> {
    fn default() -> Self {
        let content_type = ContentTypes::new();
        let rels = Rels::new();
        let doc_props = DocProps::new(CorePropsConfig::new());
        let styles = Styles::new();
        let document = Document::new();
        let document_rels = DocumentRels::new();
        let settings = Settings::new();
        let font_table = FontTable::new();
        let comments = Comments::new();
        Docx {
            content_type,
            rels,
            doc_props,
            styles,
            document,
            comments,
            document_rels,
            settings,
            font_table,
        }
    }
}

impl<'a> Docx<'a> {
    pub fn new() -> Docx<'a> {
        Default::default()
    }

    pub fn add_paragraph(mut self, p: Paragraph<'a>) -> Docx<'a> {
        self.document = self.document.add_paragraph(p);
        self
    }

    pub fn add_table(mut self, t: Table<'a>) -> Docx<'a> {
        self.document = self.document.add_table(t);
        self
    }

    pub fn build(&self) -> XMLDocx {
        XMLDocx {
            content_type: self.content_type.build(),
            rels: self.rels.build(),
            doc_props: self.doc_props.build(),
            styles: self.styles.build(),
            document: self.document.build(),
            document_rels: self.document_rels.build(),
            settings: self.settings.build(),
            font_table: self.font_table.build(),
        }
    }
}
