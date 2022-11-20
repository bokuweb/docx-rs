use std::{collections::HashMap, str::FromStr};

mod bookmark_id;
mod build_xml;
mod comments;
mod comments_extended;
mod content_types;
mod custom_item;
mod custom_item_property;
mod custom_item_rels;
mod doc_props;
mod document;
mod document_rels;
mod elements;
mod font_table;
mod footer;
mod footer_id;
mod header;
mod header_id;
mod history_id;
mod hyperlink_id;
mod numberings;
mod paragraph_id;
mod paragraph_property_change_id;
mod pic_id;
mod preset_styles;
mod rels;
mod settings;
mod styles;
mod taskpanes;
mod taskpanes_rels;
mod theme;
mod toc_key;
mod web_settings;
mod webextension;
mod xml_docx;

pub(crate) use build_xml::BuildXML;
pub(crate) use history_id::HistoryId;
pub(crate) use hyperlink_id::*;
use image::ImageFormat;
pub(crate) use paragraph_id::*;
pub(crate) use paragraph_property_change_id::ParagraphPropertyChangeId;
pub(crate) use pic_id::*;

pub use bookmark_id::*;
pub use comments::*;
pub use comments_extended::*;
pub use content_types::*;
pub use custom_item::*;
pub use custom_item_property::*;
pub use custom_item_rels::*;
pub use doc_props::*;
pub use document::*;
pub use document_rels::*;
pub use elements::*;
pub use font_table::*;
pub use footer::*;
pub use footer_id::*;
pub use header::*;
pub use header_id::*;
pub use numberings::*;
pub use rels::*;
pub use settings::*;
pub use styles::*;
pub use taskpanes::*;
pub use taskpanes_rels::*;
pub use theme::*;
pub use toc_key::*;
pub use web_settings::*;
pub use webextension::*;
pub use xml_docx::*;

use serde::{ser, Serialize};

#[derive(Debug, Clone)]
pub struct Image(pub Vec<u8>);

#[derive(Debug, Clone)]
pub struct Png(pub Vec<u8>);

pub type ImageIdAndPath = (String, String);
pub type ImageIdAndBuf = (String, Vec<u8>);

impl ser::Serialize for Image {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let base64 = base64::display::Base64Display::with_config(&*self.0, base64::STANDARD);
        serializer.collect_str(&base64)
    }
}

impl ser::Serialize for Png {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let base64 = base64::display::Base64Display::with_config(&*self.0, base64::STANDARD);
        serializer.collect_str(&base64)
    }
}

#[derive(Debug, Clone, Serialize)]
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
    pub media: Vec<(String, Vec<u8>)>,
    pub comments_extended: CommentsExtended,
    pub web_settings: WebSettings,
    pub taskpanes: Option<Taskpanes>,
    pub taskpanes_rels: TaskpanesRels,
    pub web_extensions: Vec<WebExtension>,
    pub custom_items: Vec<CustomItem>,
    pub custom_item_props: Vec<CustomItemProperty>,
    pub custom_item_rels: Vec<CustomItemRels>,
    // reader only
    pub themes: Vec<Theme>,
    // reader only
    pub images: Vec<(String, String, Image, Png)>,
    // reader only
    pub hyperlinks: Vec<(String, String, String)>,
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
        let comments_extended = CommentsExtended::new();
        let web_settings = WebSettings::new();

        Docx {
            content_type,
            rels,
            document_rels,
            doc_props,
            styles,
            document,
            comments,
            numberings,
            settings,
            font_table,
            media,
            comments_extended,
            web_settings,
            taskpanes: None,
            taskpanes_rels: TaskpanesRels::new(),
            web_extensions: vec![],
            custom_items: vec![],
            custom_item_props: vec![],
            custom_item_rels: vec![],
            themes: vec![],
            images: vec![],
            hyperlinks: vec![],
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

    pub fn add_style(mut self, s: Style) -> Self {
        self.styles = self.styles.add_style(s);
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

    // reader only
    pub(crate) fn add_image(
        mut self,
        id: impl Into<String>,
        path: impl Into<String>,
        buf: Vec<u8>,
    ) -> Self {
        if let Ok(dimg) = image::load_from_memory(&buf) {
            let mut png = std::io::Cursor::new(vec![]);
            // For now only png supported
            dimg.write_to(&mut png, ImageFormat::Png)
                .expect("Unable to write dynamic image");

            self.images
                .push((id.into(), path.into(), Image(buf), Png(png.into_inner())));
        }
        self
    }

    // reader only
    pub(crate) fn add_hyperlink(
        mut self,
        id: impl Into<String>,
        path: impl Into<String>,
        r#type: impl Into<String>,
    ) -> Self {
        self.hyperlinks
            .push((id.into(), path.into(), r#type.into()));
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

    pub fn add_paragraphs(mut self, paragraphs: Vec<Paragraph>) -> Self {
        for p in paragraphs.into_iter() {
            if p.has_numbering {
                self.document_rels.has_numberings = true
            }
            self.document = self.document.add_paragraph(p)
        }
        self
    }

    pub fn add_structured_data_tag(mut self, t: StructuredDataTag) -> Docx {
        if t.has_numbering {
            // If this document has numbering, set numberings.xml to document_rels.
            // This is because numberings.xml without numbering cause an error on word online.
            self.document_rels.has_numberings = true;
        }
        self.document = self.document.add_structured_data_tag(t);
        self
    }

    pub fn add_table_of_contents(mut self, t: TableOfContents) -> Docx {
        self.document = self.document.add_table_of_contents(t);
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

    pub fn header(mut self, header: Header) -> Self {
        if header.has_numbering {
            self.document_rels.has_numberings = true;
        }
        let count = self.document_rels.header_count + 1;
        self.document.section_property = self
            .document
            .section_property
            .header(header, &create_header_rid(count));
        self.document_rels.header_count = count;
        self.content_type = self.content_type.add_header();
        self
    }

    pub fn first_header(mut self, header: Header) -> Self {
        if header.has_numbering {
            self.document_rels.has_numberings = true;
        }
        let count = self.document_rels.header_count + 1;
        self.document.section_property = self
            .document
            .section_property
            .first_header(header, &create_header_rid(count));
        self.document_rels.header_count = count;
        self.content_type = self.content_type.add_header();
        self
    }

    pub fn even_header(mut self, header: Header) -> Self {
        if header.has_numbering {
            self.document_rels.has_numberings = true;
        }
        let count = self.document_rels.header_count + 1;
        self.document.section_property = self
            .document
            .section_property
            .even_header(header, &create_header_rid(count));
        self.document_rels.header_count = count;
        self.content_type = self.content_type.add_header();
        self.settings = self.settings.even_and_odd_headers();
        self
    }

    pub fn footer(mut self, footer: Footer) -> Self {
        if footer.has_numbering {
            self.document_rels.has_numberings = true;
        }
        let count = self.document_rels.footer_count + 1;
        self.document.section_property = self
            .document
            .section_property
            .footer(footer, &create_footer_rid(count));
        self.document_rels.footer_count = count;
        self.content_type = self.content_type.add_footer();
        self
    }

    pub fn first_footer(mut self, footer: Footer) -> Self {
        if footer.has_numbering {
            self.document_rels.has_numberings = true;
        }
        let count = self.document_rels.footer_count + 1;
        self.document.section_property = self
            .document
            .section_property
            .first_footer(footer, &create_footer_rid(count));
        self.document_rels.footer_count = count;
        self.content_type = self.content_type.add_footer();
        self
    }

    pub fn even_footer(mut self, footer: Footer) -> Self {
        if footer.has_numbering {
            self.document_rels.has_numberings = true;
        }
        let count = self.document_rels.footer_count + 1;
        self.document.section_property = self
            .document
            .section_property
            .even_footer(footer, &create_footer_rid(count));
        self.document_rels.footer_count = count;
        self.content_type = self.content_type.add_footer();
        self.settings = self.settings.even_and_odd_headers();
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

    pub fn custom_property(mut self, name: impl Into<String>, item: impl Into<String>) -> Self {
        self.doc_props = self.doc_props.custom_property(name, item);
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

    pub fn taskpanes(mut self) -> Self {
        self.taskpanes = Some(Taskpanes::new());
        self.rels = self.rels.add_taskpanes_rel();
        self.content_type = self.content_type.add_taskpanes();
        self
    }

    pub fn web_extension(mut self, ext: WebExtension) -> Self {
        self.web_extensions.push(ext);
        self.taskpanes_rels = self.taskpanes_rels.add_rel();
        self.content_type = self.content_type.add_web_extensions();
        self
    }

    pub fn add_custom_item(mut self, id: &str, xml: &str) -> Self {
        let x = CustomItem::from_str(xml).expect("should parse xml string");
        self.content_type = self.content_type.add_custom_xml();
        let rel = CustomItemRels::new().add_item();
        self.custom_item_props.push(CustomItemProperty::new(id));
        self.document_rels = self.document_rels.add_custom_item();
        self.custom_item_rels.push(rel);
        self.custom_items.push(x);
        self
    }

    pub fn build(mut self) -> XMLDocx {
        self.reset();

        self.update_dependencies();

        let tocs: Vec<(usize, Box<TableOfContents>)> = self
            .document
            .children
            .iter()
            .enumerate()
            .filter_map(|(i, child)| {
                if let DocumentChild::TableOfContents(toc) = child {
                    Some((i, toc.clone()))
                } else {
                    None
                }
            })
            .collect();

        if !tocs.is_empty() {
            for i in 1..=9 {
                self.styles = self
                    .styles
                    .add_style(crate::documents::preset_styles::toc(i));
            }
        }

        for (i, toc) in tocs {
            if toc.items.is_empty() && toc.auto {
                let children =
                    update_document_by_toc(self.document.children, &self.styles, *toc, i);
                self.document.children = children;
            }
        }

        let (images, images_bufs) = self.create_images();
        let web_extensions = self.web_extensions.iter().map(|ext| ext.build()).collect();
        let custom_items = self.custom_items.iter().map(|xml| xml.build()).collect();
        let custom_item_props = self.custom_item_props.iter().map(|p| p.build()).collect();
        let custom_item_rels = self
            .custom_item_rels
            .iter()
            .map(|rel| rel.build())
            .collect();

        self.document_rels.images = images;

        let headers: Vec<Vec<u8>> = self
            .document
            .section_property
            .get_headers()
            .iter()
            .map(|h| h.build())
            .collect();

        let footers: Vec<Vec<u8>> = self
            .document
            .section_property
            .get_footers()
            .iter()
            .map(|h| h.build())
            .collect();

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
            media: images_bufs,
            headers,
            footers,
            comments_extended: self.comments_extended.build(),
            taskpanes: self.taskpanes.map(|taskpanes| taskpanes.build()),
            taskpanes_rels: self.taskpanes_rels.build(),
            web_extensions,
            custom_items,
            custom_item_rels,
            custom_item_props,
        }
    }

    pub fn json(&self) -> String {
        self.reset();

        serde_json::to_string_pretty(&self).unwrap()
    }

    // Internal: for docx-wasm
    pub fn json_with_update_comments(&mut self) -> String {
        self.reset();

        self.update_dependencies();
        serde_json::to_string_pretty(&self).unwrap()
    }

    fn reset(&self) {
        crate::reset_para_id();
    }

    fn insert_comment_to_map(
        &self,
        comment_map: &mut HashMap<usize, String>,
        c: &CommentRangeStart,
    ) {
        let comment = c.get_comment();
        let comment_id = comment.id();
        for child in comment.children {
            if let CommentChild::Paragraph(child) = child {
                let para_id = child.id.clone();
                comment_map.insert(comment_id, para_id.clone());
            }
            // TODO: Support table in comment
        }
    }

    // Traverse and clone comments from document and add to comments node.
    fn update_dependencies(&mut self) {
        let mut comments: Vec<Comment> = vec![];
        let mut comments_extended: Vec<CommentExtended> = vec![];
        let mut comment_map: HashMap<usize, String> = HashMap::new();

        let mut hyperlink_map: HashMap<String, String> = HashMap::new();

        for child in &self.document.children {
            match child {
                DocumentChild::Paragraph(paragraph) => {
                    for child in &paragraph.children {
                        if let ParagraphChild::CommentStart(c) = child {
                            self.insert_comment_to_map(&mut comment_map, c);
                        }
                        if let ParagraphChild::Hyperlink(h) = child {
                            if let HyperlinkData::External { rid, path } = h.link.clone() {
                                hyperlink_map.insert(rid, path);
                            };
                            for child in &h.children {
                                if let ParagraphChild::CommentStart(c) = child {
                                    self.insert_comment_to_map(&mut comment_map, c);
                                }
                            }
                        }
                    }
                }
                DocumentChild::Table(table) => {
                    collect_dependencies_in_table(
                        table,
                        &mut comments,
                        &mut comments_extended,
                        &mut comment_map,
                        &mut hyperlink_map,
                    );
                }
                _ => {}
            }
        }

        for child in &self.document.children {
            match child {
                DocumentChild::Paragraph(paragraph) => {
                    for child in &paragraph.children {
                        if let ParagraphChild::CommentStart(c) = child {
                            push_comment_and_comment_extended(
                                &mut comments,
                                &mut comments_extended,
                                &comment_map,
                                c,
                            );
                        }
                        if let ParagraphChild::Hyperlink(h) = child {
                            if let HyperlinkData::External { rid, path } = h.link.clone() {
                                hyperlink_map.insert(rid, path);
                            };
                            for child in &h.children {
                                if let ParagraphChild::CommentStart(c) = child {
                                    push_comment_and_comment_extended(
                                        &mut comments,
                                        &mut comments_extended,
                                        &comment_map,
                                        c,
                                    );
                                }
                            }
                        }
                    }
                }
                DocumentChild::Table(table) => {
                    collect_dependencies_in_table(
                        table,
                        &mut comments,
                        &mut comments_extended,
                        &mut comment_map,
                        &mut hyperlink_map,
                    );
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

        for (id, d) in hyperlink_map {
            self.document_rels
                .hyperlinks
                .push((id, d, "External".to_string())); // Now support external only
        }
    }

    // Traverse and clone comments from document and add to comments node.
    // reader only
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
                DocumentChild::Table(table) => store_comments_in_table(table, comments),
                _ => {}
            }
        }

        if !comments.is_empty() {
            self.document_rels.has_comments = true;
        }
    }

    // Traverse and collect images from document.
    fn create_images(&mut self) -> (Vec<ImageIdAndPath>, Vec<ImageIdAndBuf>) {
        let mut images: Vec<(String, String)> = vec![];
        let mut image_bufs: Vec<(String, Vec<u8>)> = vec![];

        for child in &mut self.document.children {
            match child {
                DocumentChild::Paragraph(paragraph) => {
                    collect_images_from_paragraph(paragraph, &mut images, &mut image_bufs);
                }
                DocumentChild::Table(table) => {
                    for TableChild::TableRow(row) in &mut table.rows {
                        for TableRowChild::TableCell(cell) in &mut row.cells {
                            for content in &mut cell.children {
                                match content {
                                    TableCellContent::Paragraph(paragraph) => {
                                        collect_images_from_paragraph(
                                            paragraph,
                                            &mut images,
                                            &mut image_bufs,
                                        );
                                    }
                                    TableCellContent::Table(table) => {
                                        collect_images_from_table(
                                            table,
                                            &mut images,
                                            &mut image_bufs,
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        (images, image_bufs)
    }
}

fn collect_dependencies_in_table(
    table: &Table,
    comments: &mut Vec<Comment>,
    comments_extended: &mut Vec<CommentExtended>,
    comment_map: &mut HashMap<usize, String>,
    hyperlink_map: &mut HashMap<String, String>,
) {
    for TableChild::TableRow(row) in &table.rows {
        for TableRowChild::TableCell(cell) in &row.cells {
            for content in &cell.children {
                match content {
                    TableCellContent::Paragraph(paragraph) => {
                        for child in &paragraph.children {
                            if let ParagraphChild::CommentStart(c) = child {
                                push_comment_and_comment_extended(
                                    comments,
                                    comments_extended,
                                    comment_map,
                                    c,
                                );
                            }
                            if let ParagraphChild::Hyperlink(h) = child {
                                if let HyperlinkData::External { rid, path } = h.link.clone() {
                                    hyperlink_map.insert(rid, path);
                                };
                                for child in &h.children {
                                    if let ParagraphChild::CommentStart(c) = child {
                                        push_comment_and_comment_extended(
                                            comments,
                                            comments_extended,
                                            comment_map,
                                            c,
                                        );
                                    }
                                }
                            }
                        }
                    }
                    TableCellContent::Table(table) => collect_dependencies_in_table(
                        table,
                        comments,
                        comments_extended,
                        comment_map,
                        hyperlink_map,
                    ),
                }
            }
        }
    }
}

fn store_comments_in_table(table: &mut Table, comments: &[Comment]) {
    for TableChild::TableRow(row) in &mut table.rows {
        for TableRowChild::TableCell(cell) in &mut row.cells {
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
                    TableCellContent::Table(ref mut table) => {
                        store_comments_in_table(table, comments);
                    }
                }
            }
        }
    }
}

fn collect_images_from_paragraph(
    paragraph: &mut Paragraph,
    images: &mut Vec<(String, String)>,
    image_bufs: &mut Vec<(String, Vec<u8>)>,
) {
    for child in &mut paragraph.children {
        if let ParagraphChild::Run(run) = child {
            for child in &mut run.children {
                if let RunChild::Drawing(d) = child {
                    if let Some(DrawingData::Pic(pic)) = &mut d.data {
                        let b = std::mem::take(&mut pic.image);
                        let buf = image_bufs
                            .iter()
                            .find(|x| x.0 == pic.id.clone() || x.1 == b.clone());
                        if buf.as_ref().is_none() {
                            images.push((
                                pic.id.clone(),
                                // For now only png supported
                                format!("media/{}.png", pic.id),
                            ));
                            image_bufs.push((pic.id.clone(), b));
                        } else {
                            pic.id = buf.unwrap().0.clone();
                        }
                    }
                }
            }
        } else if let ParagraphChild::Insert(ins) = child {
            for child in &mut ins.children {
                match child {
                    InsertChild::Run(run) => {
                        for child in &mut run.children {
                            if let RunChild::Drawing(d) = child {
                                if let Some(DrawingData::Pic(pic)) = &mut d.data {
                                    images.push((
                                        pic.id.clone(),
                                        // For now only png supported
                                        format!("media/{}.png", pic.id),
                                    ));
                                    let b = std::mem::take(&mut pic.image);
                                    image_bufs.push((pic.id.clone(), b));
                                }
                            }
                        }
                    }
                    InsertChild::Delete(del) => {
                        for d in &mut del.children {
                            if let DeleteChild::Run(run) = d {
                                for child in &mut run.children {
                                    if let RunChild::Drawing(d) = child {
                                        if let Some(DrawingData::Pic(pic)) = &mut d.data {
                                            images.push((
                                                pic.id.clone(),
                                                // For now only png supported
                                                format!("media/{}.png", pic.id),
                                            ));
                                            let b = std::mem::take(&mut pic.image);
                                            image_bufs.push((pic.id.clone(), b));
                                        }
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        } else if let ParagraphChild::Delete(del) = child {
            for d in &mut del.children {
                if let DeleteChild::Run(run) = d {
                    for child in &mut run.children {
                        if let RunChild::Drawing(d) = child {
                            if let Some(DrawingData::Pic(pic)) = &mut d.data {
                                images.push((
                                    pic.id.clone(),
                                    // For now only png supported
                                    format!("media/{}.png", pic.id),
                                ));
                                let b = std::mem::take(&mut pic.image);
                                image_bufs.push((pic.id.clone(), b));
                            }
                        }
                    }
                }
            }
        }
    }
}

fn push_comment_and_comment_extended(
    comments: &mut Vec<Comment>,
    comments_extended: &mut Vec<CommentExtended>,
    comment_map: &HashMap<usize, String>,
    c: &CommentRangeStart,
) {
    let comment = c.get_comment();
    for child in comment.children {
        if let CommentChild::Paragraph(child) = child {
            let para_id = child.id.clone();
            comments.push(c.get_comment());
            let comment_extended = CommentExtended::new(para_id);
            if let Some(parent_comment_id) = comment.parent_comment_id {
                if let Some(parent_para_id) = comment_map.get(&parent_comment_id) {
                    comments_extended
                        .push(comment_extended.parent_paragraph_id(parent_para_id.clone()));
                }
            } else {
                comments_extended.push(comment_extended);
            }
        }
        // TODO: Support table in comment
    }
}

fn collect_images_from_table(
    table: &mut Table,
    images: &mut Vec<(String, String)>,
    image_bufs: &mut Vec<(String, Vec<u8>)>,
) {
    for TableChild::TableRow(row) in &mut table.rows {
        for TableRowChild::TableCell(cell) in &mut row.cells {
            for content in &mut cell.children {
                match content {
                    TableCellContent::Paragraph(paragraph) => {
                        collect_images_from_paragraph(paragraph, images, image_bufs);
                    }
                    TableCellContent::Table(table) => {
                        collect_images_from_table(table, images, image_bufs)
                    }
                }
            }
        }
    }
}

fn update_document_by_toc(
    document_children: Vec<DocumentChild>,
    styles: &Styles,
    toc: TableOfContents,
    toc_index: usize,
) -> Vec<DocumentChild> {
    let heading_map = styles.create_heading_style_map();
    let mut items = vec![];
    let mut children = vec![];
    let style_map: std::collections::HashMap<String, usize> = toc
        .instr
        .styles_with_levels
        .iter()
        .map(|sl| sl.0.clone())
        .collect();

    if toc.instr.heading_styles_range.is_none() && !toc.instr.styles_with_levels.is_empty() {
        // INFO: if \t option set without heading styles ranges, Microsoft word does not show ToC items...
        return document_children;
    }

    let (min, max) = toc.instr.heading_styles_range.unwrap_or((0, 9));

    for child in document_children.into_iter() {
        match child {
            DocumentChild::Paragraph(mut paragraph) => {
                if let Some(heading_level) = paragraph
                    .property
                    .style
                    .as_ref()
                    .map(|p| p.val.to_string())
                    .and_then(|sid| heading_map.get(&sid))
                {
                    if min <= *heading_level && max >= *heading_level {
                        let toc_key = TocKey::generate();
                        items.push(
                            TableOfContentsItem::new()
                                .text(paragraph.raw_text())
                                .toc_key(&toc_key)
                                .level(*heading_level),
                        );
                        paragraph =
                            Box::new(paragraph.wrap_by_bookmark(generate_bookmark_id(), &toc_key));
                    }

                    if let Some((_min, _max)) = toc.instr.tc_field_level_range {
                        // TODO: check tc field
                    }
                }

                // Support \t option. Collect toc items if style id matched.
                if let Some(level) = paragraph
                    .property
                    .style
                    .as_ref()
                    .and_then(|s| style_map.get(&s.val))
                {
                    if min <= *level && max >= *level {
                        let toc_key = TocKey::generate();
                        items.push(
                            TableOfContentsItem::new()
                                .text(paragraph.raw_text())
                                .toc_key(&toc_key)
                                .level(*level),
                        );
                        paragraph =
                            Box::new(paragraph.wrap_by_bookmark(generate_bookmark_id(), &toc_key));
                    }
                }

                children.push(DocumentChild::Paragraph(paragraph));
            }
            DocumentChild::Table(ref _table) => {
                // TODO:
                // for row in &table.rows {
                //     for cell in &row.cells {
                //         for content in &cell.children {
                //             match content {
                //                 TableCellContent::Paragraph(paragraph) => {}
                //                 TableCellContent::Table(_) => {
                //                     // TODO: Support table in table
                //                 }
                //             }
                //         }
                //     }
                // }
                children.push(child);
            }
            _ => {
                children.push(child);
            }
        }
    }

    let mut toc = toc;
    toc.items = items;
    children[toc_index] = DocumentChild::TableOfContents(Box::new(toc));
    children
}
