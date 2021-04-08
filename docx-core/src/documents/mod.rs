use std::collections::HashMap;

mod build_xml;
mod comments;
mod comments_extended;
mod content_types;
mod doc_props;
mod document;
mod document_rels;
mod elements;
mod font_table;
mod header;
mod header_id;
mod history_id;
mod numberings;
mod paragraph_id;
mod pic_id;
mod rels;
mod settings;
mod styles;
mod web_settings;
mod xml_docx;

pub(crate) use build_xml::BuildXML;
pub(crate) use history_id::HistoryId;
pub(crate) use paragraph_id::*;
pub(crate) use pic_id::*;

pub use comments::*;
pub use comments_extended::*;
pub use content_types::*;
pub use doc_props::*;
pub use document::*;
pub use document_rels::*;
pub use elements::*;
pub use font_table::*;
pub use header::*;
pub use numberings::*;
pub use rels::*;
pub use settings::*;
pub use styles::*;
pub use web_settings::*;
pub use xml_docx::*;

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Docx {
    pub content_type: ContentTypes,
    pub rels: Rels,
    pub document_rels: DocumentRels,
    pub doc_props: DocProps,
    pub styles: Styles,
    pub document: Document,
    pub comments: Comments,
    pub numberings: Numberings,
    pub settings: Settings,
    pub font_table: FontTable,
    pub media: Vec<(usize, Vec<u8>)>,
    pub header: Header,
    pub comments_extended: CommentsExtended,
    pub web_settings: WebSettings,
}

impl Default for Docx {
    fn default() -> Self {
        let content_type = ContentTypes::new().set_default();
        let rels = Rels::new().set_default();
        let doc_props = DocProps::new(CorePropsConfig::new());
        let styles = Styles::new();
        let document = Document::new();
        let document_rels = DocumentRels::new();
        let settings = Settings::new();
        let font_table = FontTable::new();
        let comments = Comments::new();
        let numberings = Numberings::new();
        let media = vec![];
        let header = Header::new();
        let comments_extended = CommentsExtended::new();
        let web_settings = WebSettings::new();

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
            numberings,
            media,
            header,
            comments_extended,
            web_settings
        }
    }
}

impl Docx {
    pub fn new() -> Docx {
        Default::default()
    }

    pub fn document(mut self, d: Document) -> Docx {
        for child in &self.document.children {
            match child {
                DocumentChild::Paragraph(paragraph) => {
                    if paragraph.has_numbering {
                        self.document_rels.has_numberings = true;
                    }
                }
                DocumentChild::Table(table) => {
                    if table.has_numbering {
                        self.document_rels.has_numberings = true;
                    }
                }
                _ => {}
            }
        }
        self.document = d;
        self
    }

    pub fn styles(mut self, s: Styles) -> Self {
        self.styles = s;
        self
    }

    pub fn numberings(mut self, n: Numberings) -> Self {
        self.numberings = n;
        self
    }

    pub fn settings(mut self, s: Settings) -> Self {
        self.settings = s;
        self
    }

    // reader only
    pub(crate) fn web_settings(mut self, s: WebSettings) -> Self {
        self.web_settings = s;
        self
    }

    pub fn comments(mut self, c: Comments) -> Self {
        self.comments = c;
        self
    }

    pub fn comments_extended(mut self, c: CommentsExtended) -> Self {
        self.comments_extended = c;
        self
    }

    pub fn add_paragraph(mut self, p: Paragraph) -> Docx {
        if p.has_numbering {
            // If this document has numbering, set numberings.xml to document_rels.
            // This is because numberings.xml without numbering cause an error on word online.
            self.document_rels.has_numberings = true;
        }
        self.document = self.document.add_paragraph(p);
        self
    }

    pub fn add_bookmark_start(mut self, id: usize, name: impl Into<String>) -> Docx {
        self.document = self.document.add_bookmark_start(id, name);
        self
    }

    pub fn add_bookmark_end(mut self, id: usize) -> Docx {
        self.document = self.document.add_bookmark_end(id);
        self
    }

    pub fn add_table(mut self, t: Table) -> Docx {
        if t.has_numbering {
            // If this document has numbering, set numberings.xml to document_rels.
            // This is because numberings.xml without numbering cause an error on word online.
            self.document_rels.has_numberings = true;
        }
        self.document = self.document.add_table(t);
        self
    }

    pub fn add_header_paragraph(mut self, p: Paragraph) -> Docx {
        if p.has_numbering {
            // If this document has numbering, set numberings.xml to document_rels.
            // This is because numberings.xml without numbering cause an error on word online.
            self.document_rels.has_numberings = true;
        }
        self.header = self.header.add_paragraph(p);
        self
    }

    pub fn add_abstract_numbering(mut self, num: AbstractNumbering) -> Docx {
        self.numberings = self.numberings.add_abstract_numbering(num);
        self
    }

    pub fn add_numbering(mut self, num: Numbering) -> Docx {
        self.numberings = self.numberings.add_numbering(num);
        self
    }

    pub fn created_at(mut self, date: &str) -> Self {
        self.doc_props = self.doc_props.created_at(date);
        self
    }

    pub fn updated_at(mut self, date: &str) -> Self {
        self.doc_props = self.doc_props.updated_at(date);
        self
    }

    pub fn doc_id(mut self, id: &str) -> Self {
        self.settings = self.settings.doc_id(id);
        self
    }

    pub fn default_tab_stop(mut self, stop: usize) -> Self {
        self.settings = self.settings.default_tab_stop(stop);
        self
    }

    pub fn add_doc_var(mut self, name: &str, val: &str) -> Self {
        self.settings = self.settings.add_doc_var(name, val);
        self
    }

    pub fn page_size(mut self, w: u32, h: u32) -> Self {
        self.document = self.document.page_size(PageSize::new().size(w, h));
        self
    }

    pub fn page_margin(mut self, margin: crate::types::PageMargin) -> Self {
        self.document = self.document.page_margin(margin);
        self
    }

    pub fn page_orient(mut self, o: crate::types::PageOrientationType) -> Self {
        self.document = self.document.page_orient(o);
        self
    }

    pub fn default_size(mut self, size: usize) -> Self {
        self.styles = self.styles.default_size(size);
        self
    }

    pub fn default_spacing(mut self, spacing: i32) -> Self {
        self.styles = self.styles.default_spacing(spacing);
        self
    }

    pub fn default_fonts(mut self, font: RunFonts) -> Self {
        self.styles = self.styles.default_fonts(font);
        self
    }

    pub fn build(&mut self) -> XMLDocx {
        self.reset();

        self.update_comments();
        let (image_ids, images) = self.create_images();

        self.document_rels.image_ids = image_ids;

        XMLDocx {
            content_type: self.content_type.build(),
            rels: self.rels.build(),
            doc_props: self.doc_props.build(),
            styles: self.styles.build(),
            document: self.document.build(),
            comments: self.comments.build(),
            document_rels: self.document_rels.build(),
            settings: self.settings.build(),
            font_table: self.font_table.build(),
            numberings: self.numberings.build(),
            media: images,
            header: self.header.build(),
            comments_extended: self.comments_extended.build(),
        }
    }

    pub fn json(&self) -> String {
        self.reset();

        serde_json::to_string_pretty(&self).unwrap()
    }

    // Internal: for docx-wasm
    pub fn json_with_update_comments(&mut self) -> String {
        self.reset();

        self.update_comments();
        serde_json::to_string_pretty(&self).unwrap()
    }

    fn reset(&self) {
        crate::reset_para_id();
    }

    // Traverse and clone comments from document and add to comments node.
    fn update_comments(&mut self) {
        let mut comments: Vec<Comment> = vec![];
        let mut comments_extended: Vec<CommentExtended> = vec![];

        let mut comment_map: HashMap<usize, String> = HashMap::new();
        for child in &self.document.children {
            match child {
                DocumentChild::Paragraph(paragraph) => {
                    for child in &paragraph.children {
                        if let ParagraphChild::CommentStart(c) = child {
                            let comment = c.get_comment();
                            let comment_id = comment.id();
                            for child in comment.children {
                                if let CommentChild::Paragraph(child) = child {
                                    let para_id = child.id.clone();
                                    comment_map.insert(comment_id, para_id.clone());
                                }
                                // TODO: Support table
                            }
                        }
                    }
                }
                DocumentChild::Table(table) => {
                    for row in &table.rows {
                        for cell in &row.cells {
                            for content in &cell.children {
                                match content {
                                    TableCellContent::Paragraph(paragraph) => {
                                        for child in &paragraph.children {
                                            if let ParagraphChild::CommentStart(c) = child {
                                                let comment = c.get_comment();
                                                let comment_id = comment.id();
                                                for child in comment.children {
                                                    if let CommentChild::Paragraph(child) = child {
                                                        let para_id = child.id.clone();
                                                        comment_map
                                                            .insert(comment_id, para_id.clone());
                                                    }
                                                    // TODO: Support table
                                                }
                                            }
                                        }
                                    }
                                    TableCellContent::Table(_) => {
                                        // TODO: correct comment
                                    }
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        for child in &self.document.children {
            match child {
                DocumentChild::Paragraph(paragraph) => {
                    for child in &paragraph.children {
                        if let ParagraphChild::CommentStart(c) = child {
                            let comment = c.get_comment();
                            for child in comment.children {
                                if let CommentChild::Paragraph(child) = child {
                                    let para_id = child.id.clone();
                                    comments.push(c.get_comment());
                                    let comment_extended = CommentExtended::new(para_id);
                                    if let Some(parent_comment_id) = comment.parent_comment_id {
                                        let parent_para_id =
                                            comment_map.get(&parent_comment_id).unwrap().clone();
                                        comments_extended.push(
                                            comment_extended.parent_paragraph_id(parent_para_id),
                                        );
                                    } else {
                                        comments_extended.push(comment_extended);
                                    }
                                }
                                // TODO: Support table
                            }
                        }
                    }
                }
                DocumentChild::Table(table) => {
                    for row in &table.rows {
                        for cell in &row.cells {
                            for content in &cell.children {
                                match content {
                                    TableCellContent::Paragraph(paragraph) => {
                                        for child in &paragraph.children {
                                            if let ParagraphChild::CommentStart(c) = child {
                                                let comment = c.get_comment();
                                                for child in comment.children {
                                                    if let CommentChild::Paragraph(child) = child {
                                                        let para_id = child.id.clone();
                                                        comments.push(c.get_comment());
                                                        let comment_extended =
                                                            CommentExtended::new(para_id);
                                                        if let Some(parent_comment_id) =
                                                            comment.parent_comment_id
                                                        {
                                                            let parent_para_id = comment_map
                                                                .get(&parent_comment_id)
                                                                .unwrap()
                                                                .clone();
                                                            comments_extended.push(
                                                                comment_extended
                                                                    .parent_paragraph_id(
                                                                        parent_para_id,
                                                                    ),
                                                            );
                                                        } else {
                                                            comments_extended
                                                                .push(comment_extended);
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    TableCellContent::Table(_) => {
                                        // TODO: correct comment
                                    }
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        // If this document has comments, set comments.xml to document_rels.
        // This is because comments.xml without comment cause an error on word online.
        if !comments.is_empty() {
            self.document_rels.has_comments = true;
        }

        self.comments_extended
            .add_comments_extended(comments_extended);

        self.comments.add_comments(comments);
    }

    // Traverse and clone comments from document and add to comments node.
    pub(crate) fn store_comments(&mut self, comments: &[Comment]) {
        for child in &mut self.document.children {
            match child {
                DocumentChild::Paragraph(paragraph) => {
                    for child in &mut paragraph.children {
                        if let ParagraphChild::CommentStart(ref mut c) = child {
                            let comment_id = c.get_id();
                            if let Some(comment) = comments.iter().find(|c| c.id() == comment_id) {
                                let comment = comment.clone();
                                c.as_mut().comment(comment);
                            }
                        }
                        if let ParagraphChild::Insert(ref mut insert) = child {
                            for child in &mut insert.children {
                                if let InsertChild::CommentStart(ref mut c) = child {
                                    let comment_id = c.get_id();
                                    if let Some(comment) =
                                        comments.iter().find(|c| c.id() == comment_id)
                                    {
                                        let comment = comment.clone();
                                        c.as_mut().comment(comment);
                                    }
                                }
                            }
                        }
                        if let ParagraphChild::Delete(ref mut delete) = child {
                            for child in &mut delete.children {
                                if let DeleteChild::CommentStart(ref mut c) = child {
                                    let comment_id = c.get_id();
                                    if let Some(comment) =
                                        comments.iter().find(|c| c.id() == comment_id)
                                    {
                                        let comment = comment.clone();
                                        c.as_mut().comment(comment);
                                    }
                                }
                            }
                        }
                    }
                }
                DocumentChild::Table(table) => {
                    for row in &mut table.rows {
                        for cell in &mut row.cells {
                            for content in &mut cell.children {
                                match content {
                                    TableCellContent::Paragraph(paragraph) => {
                                        for child in &mut paragraph.children {
                                            if let ParagraphChild::CommentStart(ref mut c) = child {
                                                let comment_id = c.get_id();
                                                if let Some(comment) =
                                                    comments.iter().find(|c| c.id() == comment_id)
                                                {
                                                    let comment = comment.clone();
                                                    c.as_mut().comment(comment);
                                                }
                                            }
                                            if let ParagraphChild::Insert(ref mut insert) = child {
                                                for child in &mut insert.children {
                                                    if let InsertChild::CommentStart(ref mut c) =
                                                        child
                                                    {
                                                        let comment_id = c.get_id();
                                                        if let Some(comment) = comments
                                                            .iter()
                                                            .find(|c| c.id() == comment_id)
                                                        {
                                                            let comment = comment.clone();
                                                            c.as_mut().comment(comment);
                                                        }
                                                    }
                                                }
                                            }
                                            if let ParagraphChild::Delete(ref mut delete) = child {
                                                for child in &mut delete.children {
                                                    if let DeleteChild::CommentStart(ref mut c) =
                                                        child
                                                    {
                                                        let comment_id = c.get_id();
                                                        if let Some(comment) = comments
                                                            .iter()
                                                            .find(|c| c.id() == comment_id)
                                                        {
                                                            let comment = comment.clone();
                                                            c.as_mut().comment(comment);
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    TableCellContent::Table(_) => {
                                        // TODO: support comment
                                    }
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        if !comments.is_empty() {
            self.document_rels.has_comments = true;
        }
    }

    // Traverse and collect images from document.
    fn create_images(&mut self) -> (Vec<usize>, Vec<(usize, Vec<u8>)>) {
        let mut image_ids: Vec<usize> = vec![];
        let mut images: Vec<(usize, Vec<u8>)> = vec![];

        for child in &mut self.document.children {
            match child {
                DocumentChild::Paragraph(paragraph) => {
                    for child in &mut paragraph.children {
                        if let ParagraphChild::Run(run) = child {
                            for child in &mut run.children {
                                if let RunChild::Drawing(d) = child {
                                    if let Some(DrawingData::Pic(pic)) = &mut d.data {
                                        image_ids.push(pic.id);
                                        let b = std::mem::replace(&mut pic.image, vec![]);
                                        images.push((pic.id, b));
                                    }
                                }
                            }
                        }
                    }
                }
                DocumentChild::Table(table) => {
                    for row in &mut table.rows {
                        for cell in &mut row.cells {
                            for content in &mut cell.children {
                                match content {
                                    TableCellContent::Paragraph(paragraph) => {
                                        for child in &mut paragraph.children {
                                            if let ParagraphChild::Run(run) = child {
                                                for child in &mut run.children {
                                                    if let RunChild::Drawing(d) = child {
                                                        if let Some(DrawingData::Pic(pic)) =
                                                            &mut d.data
                                                        {
                                                            image_ids.push(pic.id);
                                                            let b = std::mem::replace(
                                                                &mut pic.image,
                                                                vec![],
                                                            );
                                                            images.push((pic.id, b));
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    TableCellContent::Table(_) => {
                                        // TODO: support comment
                                    }
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        (image_ids, images)
    }
}
