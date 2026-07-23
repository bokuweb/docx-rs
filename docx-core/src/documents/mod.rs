use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

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
mod document_tree;
mod elements;
mod font_table;
mod footer;
mod footer_id;
mod footer_rels;
mod footnote_id;
mod footnotes;
mod header;
mod header_id;
mod header_rels;
mod history_id;
mod hyperlink_id;
mod image_collector;
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

pub use build_xml::BuildXML;
pub(crate) use history_id::HistoryId;
pub(crate) use hyperlink_id::*;
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
pub use footer_rels::*;
pub use footnotes::*;
pub use header::*;
pub use header_id::*;
pub use header_rels::*;
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

use base64::Engine;
use serde::{ser, Serialize};

use self::image_collector::{
    collect_document_footnotes, collect_document_part, collect_footer_part, collect_header_part,
    MediaRegistry,
};

#[derive(Debug, Clone)]
pub struct Image(pub Vec<u8>);

/// Decoded preview bytes for an entry in `Docx.images`.
///
/// For raster originals (PNG / JPEG / BMP / GIF / TIFF) decoded via the
/// `image` crate, the contents are PNG bytes. Unsupported formats such as
/// EMF are surfaced with an empty preview so downstream consumers can
/// provide their own conversion. The struct is named `Png` for backwards
/// compatibility.
#[derive(Debug, Clone)]
pub struct Png(pub Vec<u8>);

pub type ImageIdAndPath = (String, String);
pub type ImageIdAndBuf = (String, Vec<u8>);

const PNG_SIGNATURE: &[u8; 8] = &[137, 80, 78, 71, 13, 10, 26, 10];

fn is_emf(path: &str, buf: &[u8]) -> bool {
    if path.to_ascii_lowercase().ends_with(".emf") {
        return true;
    }
    // EMF files start with EMR_HEADER, whose first 4 bytes are the
    // record type 0x00000001 (little-endian) and bytes 40..44 hold
    // the signature " EMF" (0x464D4520).
    buf.len() >= 44
        && buf[0..4] == [0x01, 0x00, 0x00, 0x00]
        && buf[40..44] == [0x20, 0x45, 0x4D, 0x46]
}

impl ser::Serialize for Image {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let base64 = base64::engine::general_purpose::STANDARD.encode(&self.0);
        serializer.collect_str(&base64)
    }
}

impl ser::Serialize for Png {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let base64 = base64::engine::general_purpose::STANDARD.encode(&self.0);
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
    /// Reader-only collection of images embedded in `word/media/`.
    ///
    /// Each tuple is `(rId, media path, original bytes, preview bytes)`.
    /// The preview is PNG for raster originals decoded via the `image`
    /// crate. Unsupported formats such as EMF keep an empty preview for
    /// downstream consumers to populate. See [`Png`] for details.
    pub images: Vec<(String, String, Image, Png)>,
    // reader only
    pub hyperlinks: Vec<(String, String, String)>,
    pub footnotes: Footnotes,
}

/// Package metadata derived while normalizing a document for output.
pub(crate) struct PackageMetadata {
    pub(crate) media: Vec<ImageIdAndBuf>,
    /// Non-empty header relationships keyed by zero-based part index.
    pub(crate) header_rels: Vec<(usize, HeaderRels)>,
    /// Non-empty footer relationships keyed by zero-based part index.
    pub(crate) footer_rels: Vec<(usize, FooterRels)>,
}

/// Expands sparse relationships only through the last part that needs them.
///
/// Empty buffers preserve indexes before a non-empty part; the package writer
/// skips those sentinels instead of emitting empty relationship files.
fn build_sparse_relationships<T: BuildXML>(relationships: Vec<(usize, T)>) -> Vec<Vec<u8>> {
    let mut parts = Vec::new();
    for (index, relationships) in relationships {
        debug_assert!(
            index >= parts.len(),
            "sparse relationship indexes must be strictly increasing"
        );
        parts.resize_with(index, Vec::new);
        parts.push(relationships.build());
    }
    parts
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
        let footnotes = Footnotes::default();

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
            footnotes,
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
                DocumentChild::Table(table) if table.has_numbering => {
                    self.document_rels.has_numberings = true;
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
        self,
        id: impl Into<String>,
        path: impl Into<String>,
        buf: Vec<u8>,
    ) -> Self {
        self.add_image_with_options(id, path, buf, true)
    }

    // reader only
    pub(crate) fn add_image_with_options(
        mut self,
        id: impl Into<String>,
        path: impl Into<String>,
        buf: Vec<u8>,
        generate_preview: bool,
    ) -> Self {
        let path: String = path.into();

        if is_emf(&path, &buf) {
            self.images
                .push((id.into(), path, Image(buf), Png(Vec::new())));
            return self;
        }

        if !generate_preview {
            self.images
                .push((id.into(), path, Image(buf), Png(Vec::new())));
            return self;
        }

        if buf.starts_with(PNG_SIGNATURE) {
            self.images
                .push((id.into(), path, Image(buf.clone()), Png(buf)));
            return self;
        }

        #[cfg(feature = "image")]
        if let Ok(dimg) = image::load_from_memory(&buf) {
            let mut png = std::io::Cursor::new(vec![]);
            // For now only png supported
            dimg.write_to(&mut png, image::ImageFormat::Png)
                .expect("Unable to write dynamic image");

            self.images
                .push((id.into(), path, Image(buf), Png(png.into_inner())));
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

    /// Attaches one header without rebuilding the surrounding section.
    ///
    /// Mutating the property in place preserves pending first/even headers and
    /// footers until each receives its own relationship and content-type entry.
    fn attach_section_header(
        &mut self,
        section: &mut Section,
        header: Header,
        attach: fn(SectionProperty, Header, &str) -> SectionProperty,
    ) {
        if header.has_numbering {
            self.document_rels.has_numberings = true;
        }
        let count = self.document_rels.header_count + 1;
        let relationship_id = create_header_rid(count);
        section.property = attach(
            std::mem::take(&mut section.property),
            header,
            &relationship_id,
        );
        self.document_rels.header_count = count;
        self.content_type = std::mem::take(&mut self.content_type).add_header();
    }

    /// Attaches one footer while preserving every unprocessed section part.
    fn attach_section_footer(
        &mut self,
        section: &mut Section,
        footer: Footer,
        attach: fn(SectionProperty, Footer, &str) -> SectionProperty,
    ) {
        if footer.has_numbering {
            self.document_rels.has_numberings = true;
        }
        let count = self.document_rels.footer_count + 1;
        let relationship_id = create_footer_rid(count);
        section.property = attach(
            std::mem::take(&mut section.property),
            footer,
            &relationship_id,
        );
        self.document_rels.footer_count = count;
        self.content_type = std::mem::take(&mut self.content_type).add_footer();
    }

    /// Appends a section and registers all of its header and footer variants.
    ///
    /// Each pending part is attached independently so adding a default header
    /// cannot discard first/even headers or any footer that still needs a
    /// relationship and content-type entry.
    pub fn add_section(mut self, s: Section) -> Docx {
        if s.has_numbering {
            // If this document has numbering, set numberings.xml to document_rels.
            // This is because numberings.xml without numbering cause an error on word online.
            self.document_rels.has_numberings = true;
        }

        let mut new_section = s;

        if let Some(header) = new_section.temp_header.take() {
            self.attach_section_header(&mut new_section, header, SectionProperty::header);
        }

        if let Some(header) = new_section.temp_first_header.take() {
            self.attach_section_header(&mut new_section, header, SectionProperty::first_header);
        }

        if let Some(header) = new_section.temp_even_header.take() {
            self.attach_section_header(&mut new_section, header, SectionProperty::even_header);
        }

        if let Some(footer) = new_section.temp_footer.take() {
            self.attach_section_footer(&mut new_section, footer, SectionProperty::footer);
        }

        if let Some(footer) = new_section.temp_first_footer.take() {
            self.attach_section_footer(&mut new_section, footer, SectionProperty::first_footer);
        }

        if let Some(footer) = new_section.temp_even_footer.take() {
            self.attach_section_footer(&mut new_section, footer, SectionProperty::even_footer);
        }

        self.document = self.document.add_section(new_section);
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

    pub fn title_pg(mut self) -> Self {
        self.document = self.document.title_pg();
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

    pub fn default_line_spacing(mut self, spacing: LineSpacing) -> Self {
        self.styles = self.styles.default_line_spacing(spacing);
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

    pub fn page_num_type(mut self, p: PageNumType) -> Self {
        self.document = self.document.page_num_type(p);
        self
    }

    /// Finalizes document-wide identifiers and relationships and renders the
    /// uncompressed OPC package parts.
    ///
    /// This consumes the document. Call [`XMLDocx::pack`] to write a DOCX
    /// archive.
    pub fn build(mut self) -> XMLDocx {
        let package = self.prepare_package();

        let web_extensions = self.web_extensions.iter().map(|ext| ext.build()).collect();
        let custom_items = self.custom_items.iter().map(|xml| xml.build()).collect();
        let custom_item_props = self.custom_item_props.iter().map(|p| p.build()).collect();
        let custom_item_rels = self
            .custom_item_rels
            .iter()
            .map(|rel| rel.build())
            .collect();

        let headers = self
            .document
            .headers()
            .map(|(_, header)| header.build())
            .collect();

        let footers = self
            .document
            .footers()
            .map(|(_, footer)| footer.build())
            .collect();

        let header_rels = build_sparse_relationships(package.header_rels);
        let footer_rels = build_sparse_relationships(package.footer_rels);

        XMLDocx {
            content_type: self.content_type.build(),
            rels: self.rels.build(),
            doc_props: self.doc_props.build(),
            styles: self.styles.build(),
            document: self.document.build(),
            comments: self.comments.build(),
            document_rels: self.document_rels.build(),
            header_rels,
            footer_rels,
            settings: self.settings.build(),
            font_table: self.font_table.build(),
            numberings: self.numberings.build(),
            media: package.media,
            headers,
            footers,
            comments_extended: self.comments_extended.build(),
            taskpanes: self.taskpanes.map(|taskpanes| taskpanes.build()),
            taskpanes_rels: self.taskpanes_rels.build(),
            web_extensions,
            custom_items,
            custom_item_rels,
            custom_item_props,
            footnotes: self.footnotes.build(),
        }
    }

    /// Writes this document directly as a DOCX ZIP archive.
    ///
    /// Unlike [`Docx::build`] followed by [`XMLDocx::pack`], this method
    /// streams XML package parts into the archive instead of retaining every
    /// rendered part in a separate `Vec<u8>`.
    pub fn pack<W>(mut self, writer: W) -> zip::result::ZipResult<()>
    where
        W: std::io::Write + std::io::Seek,
    {
        let package = self.prepare_package();
        crate::zipper::zip_docx(writer, self, package)
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

    // Internal: for docx-wasm
    pub fn comments_json(&mut self) -> String {
        self.reset();
        self.update_dependencies();
        serde_json::to_string_pretty(&self.comments).unwrap()
    }

    fn reset(&self) {
        crate::reset_para_id();
    }

    /// Expands automatically generated tables of contents before dependency
    /// collection.
    ///
    /// Discovery records only indexes so static tables of contents are not
    /// cloned. Eligible tables are moved out when expanded, while `has_toc`
    /// still reports every table of contents so preset TOC styles are added.
    fn expand_auto_tocs(&mut self) -> bool {
        let mut has_toc = false;
        let auto_toc_indexes: Vec<usize> = self
            .document
            .children
            .iter()
            .enumerate()
            .filter_map(|(index, child)| {
                let DocumentChild::TableOfContents(toc) = child else {
                    return None;
                };
                has_toc = true;
                (toc.items.is_empty() && toc.auto).then_some(index)
            })
            .collect();

        for index in auto_toc_indexes {
            let mut children = std::mem::take(&mut self.document.children);
            let toc = match std::mem::replace(
                &mut children[index],
                DocumentChild::TableOfContents(Box::default()),
            ) {
                DocumentChild::TableOfContents(toc) => *toc,
                _ => unreachable!("collected index must still contain a table of contents"),
            };
            self.document.children = update_document_by_toc(children, &self.styles, toc, index);
        }

        has_toc
    }

    /// Prepares the complete document tree for package-part rendering.
    fn prepare_for_build(&mut self) -> bool {
        self.reset();
        let has_toc = self.expand_auto_tocs();
        self.refresh_duplicate_para_ids();
        self.update_dependencies();
        has_toc
    }

    /// Normalizes the document and collects metadata shared by buffered and
    /// streaming package output.
    fn prepare_package(&mut self) -> PackageMetadata {
        let has_toc = self.prepare_for_build();

        let mut media = MediaRegistry::default();
        let document_part = collect_document_part(&mut self.document, &mut media);
        let header_images = self.collect_header_images(&mut media);
        let footer_images = self.collect_footer_images(&mut media);
        self.document_rels.images = document_part.relationships;

        let header_rels = header_images
            .into_iter()
            .map(|(index, images)| {
                let mut rels = HeaderRels::new();
                rels.set_images(images);
                (index, rels)
            })
            .collect();

        let footer_rels = footer_images
            .into_iter()
            .map(|(index, images)| {
                let mut rels = FooterRels::new();
                rels.set_images(images);
                (index, rels)
            })
            .collect();

        if !document_part.footnotes.is_empty() {
            self.footnotes.add(document_part.footnotes);
            self.content_type = std::mem::take(&mut self.content_type).add_footnotes();
            self.document_rels.has_footnotes = true;
        }

        if has_toc {
            for level in 1..=9 {
                if !self
                    .styles
                    .styles
                    .iter()
                    .any(|style| style.name == Name::new(format!("toc {level}")))
                {
                    self.styles = std::mem::take(&mut self.styles)
                        .add_style(crate::documents::preset_styles::toc(level));
                }
            }
        }

        PackageMetadata {
            media: media.into_media(),
            header_rels,
            footer_rels,
        }
    }

    /// Replaces empty and duplicate paragraph IDs with unique values.
    fn refresh_duplicate_para_ids(&mut self) {
        let mut counts: HashMap<&str, usize> = HashMap::new();
        collect_para_ids_in_docx(self, &mut counts);

        let duplicates: HashSet<String> = counts
            .iter()
            .filter(|(_, count)| **count > 1)
            .map(|(id, _)| (*id).to_owned())
            .collect();
        let has_empty_id = counts.contains_key("");
        if duplicates.is_empty() && !has_empty_id {
            return;
        }

        let mut used: HashSet<String> = counts.into_keys().map(str::to_owned).collect();
        let mut seen: HashSet<String> = HashSet::new();

        refresh_para_ids_in_docx(self, &duplicates, &mut used, &mut seen);
    }

    fn insert_comment_to_map(
        &self,
        comment_map: &mut HashMap<usize, String>,
        c: &CommentRangeStart,
    ) {
        let comment = c.get_comment_ref();
        let comment_id = comment.id();
        for child in &comment.children {
            if let CommentChild::Paragraph(child) = child {
                let para_id = child.id.clone();
                comment_map.insert(comment_id, para_id);
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
                            if let HyperlinkData::External { rid, path } = &h.link {
                                hyperlink_map.insert(rid.clone(), path.clone());
                            }
                            for child in &h.children {
                                if let ParagraphChild::CommentStart(c) = child {
                                    self.insert_comment_to_map(&mut comment_map, c);
                                }
                            }
                        }
                    }
                }
                DocumentChild::Table(table) => {
                    collect_comment_map_in_table(table, &mut comment_map, &mut hyperlink_map);
                }
                DocumentChild::TableOfContents(toc) => {
                    for child in &toc.before_contents {
                        if let TocContent::Paragraph(paragraph) = child {
                            collect_comment_map_in_paragraph(
                                paragraph,
                                &mut comment_map,
                                &mut hyperlink_map,
                            );
                        }
                        if let TocContent::Table(table) = child {
                            collect_comment_map_in_table(
                                table,
                                &mut comment_map,
                                &mut hyperlink_map,
                            );
                        }
                    }
                    for child in &toc.after_contents {
                        if let TocContent::Paragraph(paragraph) = child {
                            collect_comment_map_in_paragraph(
                                paragraph,
                                &mut comment_map,
                                &mut hyperlink_map,
                            );
                        }
                        if let TocContent::Table(table) = child {
                            collect_comment_map_in_table(
                                table,
                                &mut comment_map,
                                &mut hyperlink_map,
                            );
                        }
                    }
                }
                _ => {}
            }
        }

        if comment_map.is_empty() {
            for (id, path) in hyperlink_map {
                self.document_rels
                    .hyperlinks
                    .push((id, path, "External".to_owned()));
            }
            return;
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
                            if let HyperlinkData::External { rid, path } = &h.link {
                                hyperlink_map.insert(rid.clone(), path.clone());
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
                DocumentChild::TableOfContents(toc) => {
                    // TODO:refine later
                    for child in &toc.before_contents {
                        if let TocContent::Paragraph(paragraph) = child {
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
                                    if let HyperlinkData::External { rid, path } = &h.link {
                                        hyperlink_map.insert(rid.clone(), path.clone());
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
                        if let TocContent::Table(table) = child {
                            collect_dependencies_in_table(
                                table,
                                &mut comments,
                                &mut comments_extended,
                                &mut comment_map,
                                &mut hyperlink_map,
                            );
                        }
                    }
                    for child in &toc.after_contents {
                        if let TocContent::Paragraph(paragraph) = child {
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
                                    if let HyperlinkData::External { rid, path } = &h.link {
                                        hyperlink_map.insert(rid.clone(), path.clone());
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
                        if let TocContent::Table(table) = child {
                            collect_dependencies_in_table(
                                table,
                                &mut comments,
                                &mut comments_extended,
                                &mut comment_map,
                                &mut hyperlink_map,
                            );
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

        for (id, d) in hyperlink_map {
            self.document_rels
                .hyperlinks
                .push((id, d, "External".to_string())); // Now support external only
        }
    }

    // Traverse and clone comments from document and add to comments node.
    // reader only
    pub(crate) fn store_comments(&mut self, comments: &[Comment]) {
        let comments_by_id: HashMap<usize, &Comment> = comments
            .iter()
            .map(|comment| (comment.id(), comment))
            .collect();
        for child in &mut self.document.children {
            match child {
                DocumentChild::Paragraph(paragraph) => {
                    for child in &mut paragraph.children {
                        if let ParagraphChild::CommentStart(ref mut c) = child {
                            let comment_id = c.get_id();
                            if let Some(comment) = comments_by_id.get(&comment_id).copied() {
                                let comment = Comment::clone(comment);
                                c.as_mut().comment(comment);
                            }
                        }
                        if let ParagraphChild::Insert(ref mut insert) = child {
                            for child in &mut insert.children {
                                if let InsertChild::CommentStart(ref mut c) = child {
                                    let comment_id = c.get_id();
                                    if let Some(comment) = comments_by_id.get(&comment_id).copied()
                                    {
                                        let comment = Comment::clone(comment);
                                        c.as_mut().comment(comment);
                                    }
                                } else if let InsertChild::Delete(ref mut d) = child {
                                    for child in &mut d.children {
                                        if let DeleteChild::CommentStart(ref mut c) = child {
                                            let comment_id = c.get_id();
                                            if let Some(comment) =
                                                comments_by_id.get(&comment_id).copied()
                                            {
                                                let comment = Comment::clone(comment);
                                                c.as_mut().comment(comment);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        if let ParagraphChild::Delete(ref mut delete) = child {
                            for child in &mut delete.children {
                                if let DeleteChild::CommentStart(ref mut c) = child {
                                    let comment_id = c.get_id();
                                    if let Some(comment) = comments_by_id.get(&comment_id).copied()
                                    {
                                        let comment = Comment::clone(comment);
                                        c.as_mut().comment(comment);
                                    }
                                }
                            }
                        }
                    }
                }
                DocumentChild::Table(table) => store_comments_in_table(table, &comments_by_id),
                _ => {}
            }
        }

        if !comments.is_empty() {
            self.document_rels.has_comments = true;
        }
    }

    /// Collects image relationships for every header in package order.
    ///
    /// Each entry deliberately uses a separate relationship scope while all
    /// entries share the package-wide media registry. Relationship creation
    /// order matches the order used to render and write header parts.
    fn collect_header_images(
        &mut self,
        media: &mut MediaRegistry,
    ) -> Vec<(usize, Vec<ImageIdAndPath>)> {
        self.document
            .headers_mut()
            .enumerate()
            .filter_map(|(index, (_, header))| {
                let relationships = collect_header_part(header, media).relationships;
                (!relationships.is_empty()).then_some((index, relationships))
            })
            .collect()
    }

    /// Collects image relationships for every footer in package order.
    ///
    /// Relationship creation order matches the package writer. A shared
    /// registry prevents repeated footer and body images from being emitted as
    /// separate physical media files.
    fn collect_footer_images(
        &mut self,
        media: &mut MediaRegistry,
    ) -> Vec<(usize, Vec<ImageIdAndPath>)> {
        self.document
            .footers_mut()
            .enumerate()
            .filter_map(|(index, (_, footer))| {
                let relationships = collect_footer_part(footer, media).relationships;
                (!relationships.is_empty()).then_some((index, relationships))
            })
            .collect()
    }

    /// Collects footnote references from the complete main document tree.
    ///
    /// This method remains available for callers that collect dependencies
    /// explicitly. Normal package builds collect footnotes together with
    /// images, avoiding a second traversal of the document tree.
    pub fn collect_footnotes(&mut self) -> bool {
        let footnotes = collect_document_footnotes(&mut self.document);
        let is_footnotes = !footnotes.is_empty();
        self.footnotes.add(footnotes);
        is_footnotes
    }
}

fn collect_dependencies_in_paragraph(
    paragraph: &Paragraph,
    comments: &mut Vec<Comment>,
    comments_extended: &mut Vec<CommentExtended>,
    comment_map: &mut HashMap<usize, String>,
    hyperlink_map: &mut HashMap<String, String>,
) {
    for child in &paragraph.children {
        if let ParagraphChild::CommentStart(c) = child {
            push_comment_and_comment_extended(comments, comments_extended, comment_map, c);
        }
        if let ParagraphChild::Hyperlink(h) = child {
            if let HyperlinkData::External { rid, path } = &h.link {
                hyperlink_map.insert(rid.clone(), path.clone());
            };
            for child in &h.children {
                if let ParagraphChild::CommentStart(c) = child {
                    push_comment_and_comment_extended(comments, comments_extended, comment_map, c);
                }
            }
        }
    }
}

fn collect_comment_map_in_paragraph(
    paragraph: &Paragraph,
    comment_map: &mut HashMap<usize, String>,
    hyperlink_map: &mut HashMap<String, String>,
) {
    for child in &paragraph.children {
        if let ParagraphChild::CommentStart(c) = child {
            let comment = c.get_comment_ref();
            let comment_id = comment.id();
            for child in &comment.children {
                if let CommentChild::Paragraph(child) = child {
                    comment_map.insert(comment_id, child.id.clone());
                }
            }
        }
        if let ParagraphChild::Hyperlink(h) = child {
            if let HyperlinkData::External { rid, path } = &h.link {
                hyperlink_map.insert(rid.clone(), path.clone());
            }
            for child in &h.children {
                if let ParagraphChild::CommentStart(c) = child {
                    let comment = c.get_comment_ref();
                    let comment_id = comment.id();
                    for child in &comment.children {
                        if let CommentChild::Paragraph(child) = child {
                            comment_map.insert(comment_id, child.id.clone());
                        }
                    }
                }
            }
        }
    }
}

fn collect_comment_map_in_table(
    table: &Table,
    comment_map: &mut HashMap<usize, String>,
    hyperlink_map: &mut HashMap<String, String>,
) {
    for TableChild::TableRow(row) in &table.rows {
        for TableRowChild::TableCell(cell) in &row.cells {
            for content in &cell.children {
                match content {
                    TableCellContent::Paragraph(paragraph) => {
                        collect_comment_map_in_paragraph(paragraph, comment_map, hyperlink_map);
                    }
                    TableCellContent::Table(table) => {
                        collect_comment_map_in_table(table, comment_map, hyperlink_map)
                    }
                    TableCellContent::StructuredDataTag(tag) => {
                        for child in &tag.children {
                            if let StructuredDataTagChild::Paragraph(paragraph) = child {
                                collect_comment_map_in_paragraph(
                                    paragraph,
                                    comment_map,
                                    hyperlink_map,
                                );
                            }
                            if let StructuredDataTagChild::Table(table) = child {
                                collect_comment_map_in_table(table, comment_map, hyperlink_map);
                            }
                        }
                    }
                    TableCellContent::TableOfContents(t) => {
                        for child in &t.before_contents {
                            if let TocContent::Paragraph(paragraph) = child {
                                collect_comment_map_in_paragraph(
                                    paragraph,
                                    comment_map,
                                    hyperlink_map,
                                );
                            }
                            if let TocContent::Table(table) = child {
                                collect_comment_map_in_table(table, comment_map, hyperlink_map);
                            }
                        }
                        for child in &t.after_contents {
                            if let TocContent::Paragraph(paragraph) = child {
                                collect_comment_map_in_paragraph(
                                    paragraph,
                                    comment_map,
                                    hyperlink_map,
                                );
                            }
                            if let TocContent::Table(table) = child {
                                collect_comment_map_in_table(table, comment_map, hyperlink_map);
                            }
                        }
                    }
                }
            }
        }
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
                        collect_dependencies_in_paragraph(
                            paragraph,
                            comments,
                            comments_extended,
                            comment_map,
                            hyperlink_map,
                        );
                    }
                    TableCellContent::Table(table) => collect_dependencies_in_table(
                        table,
                        comments,
                        comments_extended,
                        comment_map,
                        hyperlink_map,
                    ),
                    TableCellContent::StructuredDataTag(tag) => {
                        for child in &tag.children {
                            if let StructuredDataTagChild::Paragraph(paragraph) = child {
                                collect_dependencies_in_paragraph(
                                    paragraph,
                                    comments,
                                    comments_extended,
                                    comment_map,
                                    hyperlink_map,
                                );
                            }
                            if let StructuredDataTagChild::Table(table) = child {
                                collect_dependencies_in_table(
                                    table,
                                    comments,
                                    comments_extended,
                                    comment_map,
                                    hyperlink_map,
                                );
                            }
                        }
                    }
                    TableCellContent::TableOfContents(t) => {
                        for child in &t.before_contents {
                            if let TocContent::Paragraph(paragraph) = child {
                                collect_dependencies_in_paragraph(
                                    paragraph,
                                    comments,
                                    comments_extended,
                                    comment_map,
                                    hyperlink_map,
                                );
                            }
                            if let TocContent::Table(table) = child {
                                collect_dependencies_in_table(
                                    table,
                                    comments,
                                    comments_extended,
                                    comment_map,
                                    hyperlink_map,
                                );
                            }
                        }

                        for child in &t.after_contents {
                            if let TocContent::Paragraph(paragraph) = child {
                                collect_dependencies_in_paragraph(
                                    paragraph,
                                    comments,
                                    comments_extended,
                                    comment_map,
                                    hyperlink_map,
                                );
                            }
                            if let TocContent::Table(table) = child {
                                collect_dependencies_in_table(
                                    table,
                                    comments,
                                    comments_extended,
                                    comment_map,
                                    hyperlink_map,
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}

fn store_comments_in_paragraph(
    paragraph: &mut Paragraph,
    comments_by_id: &HashMap<usize, &Comment>,
) {
    for child in &mut paragraph.children {
        if let ParagraphChild::CommentStart(ref mut c) = child {
            let comment_id = c.get_id();
            if let Some(comment) = comments_by_id.get(&comment_id).copied() {
                let comment = Comment::clone(comment);
                c.as_mut().comment(comment);
            }
        }
        if let ParagraphChild::Insert(ref mut insert) = child {
            for child in &mut insert.children {
                if let InsertChild::CommentStart(ref mut c) = child {
                    let comment_id = c.get_id();
                    if let Some(comment) = comments_by_id.get(&comment_id).copied() {
                        let comment = Comment::clone(comment);
                        c.as_mut().comment(comment);
                    }
                }
            }
        }
        if let ParagraphChild::Delete(ref mut delete) = child {
            for child in &mut delete.children {
                if let DeleteChild::CommentStart(ref mut c) = child {
                    let comment_id = c.get_id();
                    if let Some(comment) = comments_by_id.get(&comment_id).copied() {
                        let comment = Comment::clone(comment);
                        c.as_mut().comment(comment);
                    }
                }
            }
        }
    }
}

fn store_comments_in_table(table: &mut Table, comments_by_id: &HashMap<usize, &Comment>) {
    for TableChild::TableRow(row) in &mut table.rows {
        for TableRowChild::TableCell(cell) in &mut row.cells {
            for content in &mut cell.children {
                match content {
                    TableCellContent::Paragraph(paragraph) => {
                        store_comments_in_paragraph(paragraph, comments_by_id)
                    }
                    TableCellContent::Table(ref mut table) => {
                        store_comments_in_table(table, comments_by_id);
                    }
                    TableCellContent::StructuredDataTag(ref mut tag) => {
                        for child in &mut tag.children {
                            if let StructuredDataTagChild::Paragraph(paragraph) = child {
                                store_comments_in_paragraph(paragraph, comments_by_id);
                            }
                            if let StructuredDataTagChild::Table(table) = child {
                                store_comments_in_table(table, comments_by_id);
                            }
                        }
                    }
                    TableCellContent::TableOfContents(ref mut t) => {
                        for child in &mut t.before_contents {
                            if let TocContent::Paragraph(paragraph) = child {
                                store_comments_in_paragraph(paragraph, comments_by_id);
                            }
                            if let TocContent::Table(table) = child {
                                store_comments_in_table(table, comments_by_id);
                            }
                        }

                        for child in &mut t.after_contents {
                            if let TocContent::Paragraph(paragraph) = child {
                                store_comments_in_paragraph(paragraph, comments_by_id);
                            }
                            if let TocContent::Table(table) = child {
                                store_comments_in_table(table, comments_by_id);
                            }
                        }
                    }
                }
            }
        }
    }
}

fn collect_para_ids_in_docx<'a>(docx: &'a Docx, counts: &mut HashMap<&'a str, usize>) {
    for child in &docx.document.children {
        collect_para_ids_in_document_child(child, counts);
    }
    collect_para_ids_in_section_property(&docx.document.section_property, counts);

    for comment in &docx.comments.comments {
        collect_para_ids_in_comment(comment, counts);
    }

    for footnote in &docx.footnotes.footnotes {
        for paragraph in &footnote.content {
            collect_para_ids_in_paragraph(paragraph, counts);
        }
    }
}

fn collect_para_ids_in_document_child<'a>(
    child: &'a DocumentChild,
    counts: &mut HashMap<&'a str, usize>,
) {
    match child {
        DocumentChild::Paragraph(paragraph) => collect_para_ids_in_paragraph(paragraph, counts),
        DocumentChild::Table(table) => collect_para_ids_in_table(table, counts),
        DocumentChild::StructuredDataTag(tag) => {
            collect_para_ids_in_structured_data_tag(tag, counts)
        }
        DocumentChild::TableOfContents(toc) => collect_para_ids_in_toc(toc, counts),
        DocumentChild::Section(section) => collect_para_ids_in_section(section, counts),
        DocumentChild::CommentStart(c) => collect_para_ids_in_comment(&c.comment, counts),
        _ => {}
    }
}

fn collect_para_ids_in_section<'a>(section: &'a Section, counts: &mut HashMap<&'a str, usize>) {
    for child in &section.children {
        match child {
            SectionChild::Paragraph(paragraph) => collect_para_ids_in_paragraph(paragraph, counts),
            SectionChild::Table(table) => collect_para_ids_in_table(table, counts),
            SectionChild::StructuredDataTag(tag) => {
                collect_para_ids_in_structured_data_tag(tag, counts)
            }
            SectionChild::TableOfContents(toc) => collect_para_ids_in_toc(toc, counts),
            SectionChild::CommentStart(c) => collect_para_ids_in_comment(&c.comment, counts),
            _ => {}
        }
    }
    collect_para_ids_in_section_property(&section.property, counts);
}

fn collect_para_ids_in_section_property<'a>(
    property: &'a SectionProperty,
    counts: &mut HashMap<&'a str, usize>,
) {
    if let Some((_, header)) = property.header.as_ref() {
        collect_para_ids_in_header(header, counts);
    }
    if let Some((_, header)) = property.first_header.as_ref() {
        collect_para_ids_in_header(header, counts);
    }
    if let Some((_, header)) = property.even_header.as_ref() {
        collect_para_ids_in_header(header, counts);
    }
    if let Some((_, footer)) = property.footer.as_ref() {
        collect_para_ids_in_footer(footer, counts);
    }
    if let Some((_, footer)) = property.first_footer.as_ref() {
        collect_para_ids_in_footer(footer, counts);
    }
    if let Some((_, footer)) = property.even_footer.as_ref() {
        collect_para_ids_in_footer(footer, counts);
    }
}

fn collect_para_ids_in_header<'a>(header: &'a Header, counts: &mut HashMap<&'a str, usize>) {
    for child in &header.children {
        match child {
            HeaderChild::Paragraph(paragraph) => collect_para_ids_in_paragraph(paragraph, counts),
            HeaderChild::Table(table) => collect_para_ids_in_table(table, counts),
            HeaderChild::StructuredDataTag(tag) => {
                collect_para_ids_in_structured_data_tag(tag, counts)
            }
        }
    }
}

fn collect_para_ids_in_footer<'a>(footer: &'a Footer, counts: &mut HashMap<&'a str, usize>) {
    for child in &footer.children {
        match child {
            FooterChild::Paragraph(paragraph) => collect_para_ids_in_paragraph(paragraph, counts),
            FooterChild::Table(table) => collect_para_ids_in_table(table, counts),
            FooterChild::StructuredDataTag(tag) => {
                collect_para_ids_in_structured_data_tag(tag, counts)
            }
        }
    }
}

fn collect_para_ids_in_toc<'a>(toc: &'a TableOfContents, counts: &mut HashMap<&'a str, usize>) {
    for child in &toc.before_contents {
        match child {
            TocContent::Paragraph(paragraph) => collect_para_ids_in_paragraph(paragraph, counts),
            TocContent::Table(table) => collect_para_ids_in_table(table, counts),
        }
    }
    for child in &toc.after_contents {
        match child {
            TocContent::Paragraph(paragraph) => collect_para_ids_in_paragraph(paragraph, counts),
            TocContent::Table(table) => collect_para_ids_in_table(table, counts),
        }
    }
}

fn collect_para_ids_in_table<'a>(table: &'a Table, counts: &mut HashMap<&'a str, usize>) {
    for TableChild::TableRow(row) in &table.rows {
        for TableRowChild::TableCell(cell) in &row.cells {
            for content in &cell.children {
                match content {
                    TableCellContent::Paragraph(paragraph) => {
                        collect_para_ids_in_paragraph(paragraph, counts)
                    }
                    TableCellContent::Table(table) => collect_para_ids_in_table(table, counts),
                    TableCellContent::StructuredDataTag(tag) => {
                        collect_para_ids_in_structured_data_tag(tag, counts)
                    }
                    TableCellContent::TableOfContents(toc) => collect_para_ids_in_toc(toc, counts),
                }
            }
        }
    }
}

fn collect_para_ids_in_paragraph<'a>(
    paragraph: &'a Paragraph,
    counts: &mut HashMap<&'a str, usize>,
) {
    *counts.entry(paragraph.id.as_str()).or_insert(0) += 1;

    for child in &paragraph.children {
        match child {
            ParagraphChild::Run(run) => collect_para_ids_in_run(run, counts),
            ParagraphChild::CommentStart(c) => collect_para_ids_in_comment(&c.comment, counts),
            ParagraphChild::Insert(insert) => collect_para_ids_in_insert(insert, counts),
            ParagraphChild::Delete(delete) => collect_para_ids_in_delete(delete, counts),
            ParagraphChild::MoveFrom(moved) => collect_para_ids_in_move_from(moved, counts),
            ParagraphChild::MoveTo(moved) => collect_para_ids_in_move_to(moved, counts),
            ParagraphChild::Hyperlink(hyperlink) => {
                collect_para_ids_in_hyperlink(hyperlink, counts)
            }
            ParagraphChild::StructuredDataTag(tag) => {
                collect_para_ids_in_structured_data_tag(tag, counts)
            }
            _ => {}
        }
    }
}

fn collect_para_ids_in_hyperlink<'a>(
    hyperlink: &'a Hyperlink,
    counts: &mut HashMap<&'a str, usize>,
) {
    for child in &hyperlink.children {
        match child {
            ParagraphChild::Run(run) => collect_para_ids_in_run(run, counts),
            ParagraphChild::CommentStart(c) => collect_para_ids_in_comment(&c.comment, counts),
            ParagraphChild::Insert(insert) => collect_para_ids_in_insert(insert, counts),
            ParagraphChild::Delete(delete) => collect_para_ids_in_delete(delete, counts),
            ParagraphChild::MoveFrom(moved) => collect_para_ids_in_move_from(moved, counts),
            ParagraphChild::MoveTo(moved) => collect_para_ids_in_move_to(moved, counts),
            ParagraphChild::Hyperlink(hyperlink) => {
                collect_para_ids_in_hyperlink(hyperlink, counts)
            }
            ParagraphChild::StructuredDataTag(tag) => {
                collect_para_ids_in_structured_data_tag(tag, counts)
            }
            _ => {}
        }
    }
}

fn collect_para_ids_in_insert<'a>(insert: &'a Insert, counts: &mut HashMap<&'a str, usize>) {
    for child in &insert.children {
        match child {
            InsertChild::Run(run) => collect_para_ids_in_run(run, counts),
            InsertChild::CommentStart(c) => collect_para_ids_in_comment(&c.comment, counts),
            InsertChild::Delete(delete) => collect_para_ids_in_delete(delete, counts),
            _ => {}
        }
    }
}

fn collect_para_ids_in_delete<'a>(delete: &'a Delete, counts: &mut HashMap<&'a str, usize>) {
    for child in &delete.children {
        match child {
            DeleteChild::Run(run) => collect_para_ids_in_run(run, counts),
            DeleteChild::CommentStart(c) => collect_para_ids_in_comment(&c.comment, counts),
            DeleteChild::CommentEnd(_) => {}
        }
    }
}

fn collect_para_ids_in_move_from<'a>(moved: &'a MoveFrom, counts: &mut HashMap<&'a str, usize>) {
    for child in &moved.children {
        match child {
            MoveFromChild::Run(run) => collect_para_ids_in_run(run, counts),
            MoveFromChild::CommentStart(c) => collect_para_ids_in_comment(&c.comment, counts),
            MoveFromChild::CommentEnd(_) => {}
        }
    }
}

fn collect_para_ids_in_move_to<'a>(moved: &'a MoveTo, counts: &mut HashMap<&'a str, usize>) {
    for child in &moved.children {
        match child {
            MoveToChild::Run(run) => collect_para_ids_in_run(run, counts),
            MoveToChild::Delete(delete) => collect_para_ids_in_delete(delete, counts),
            MoveToChild::CommentStart(c) => collect_para_ids_in_comment(&c.comment, counts),
            _ => {}
        }
    }
}

fn collect_para_ids_in_run<'a>(run: &'a Run, counts: &mut HashMap<&'a str, usize>) {
    for child in &run.children {
        if let RunChild::CommentStart(c) = child {
            collect_para_ids_in_comment(&c.comment, counts);
        }
    }
}

fn collect_para_ids_in_structured_data_tag<'a>(
    tag: &'a StructuredDataTag,
    counts: &mut HashMap<&'a str, usize>,
) {
    for child in &tag.children {
        match child {
            StructuredDataTagChild::Run(run) => collect_para_ids_in_run(run, counts),
            StructuredDataTagChild::Paragraph(paragraph) => {
                collect_para_ids_in_paragraph(paragraph, counts)
            }
            StructuredDataTagChild::Table(table) => collect_para_ids_in_table(table, counts),
            StructuredDataTagChild::CommentStart(c) => {
                collect_para_ids_in_comment(&c.comment, counts)
            }
            StructuredDataTagChild::StructuredDataTag(inner) => {
                collect_para_ids_in_structured_data_tag(inner, counts)
            }
            _ => {}
        }
    }
}

fn collect_para_ids_in_comment<'a>(comment: &'a Comment, counts: &mut HashMap<&'a str, usize>) {
    for child in &comment.children {
        match child {
            CommentChild::Paragraph(paragraph) => collect_para_ids_in_paragraph(paragraph, counts),
            CommentChild::Table(table) => collect_para_ids_in_table(table, counts),
        }
    }
}

fn refresh_para_ids_in_docx(
    docx: &mut Docx,
    duplicates: &HashSet<String>,
    used: &mut HashSet<String>,
    seen: &mut HashSet<String>,
) {
    for child in &mut docx.document.children {
        refresh_para_ids_in_document_child(child, duplicates, used, seen);
    }
    refresh_para_ids_in_section_property(
        &mut docx.document.section_property,
        duplicates,
        used,
        seen,
    );

    for comment in &mut docx.comments.comments {
        refresh_para_ids_in_comment(comment, duplicates, used, seen);
    }

    for footnote in &mut docx.footnotes.footnotes {
        for paragraph in &mut footnote.content {
            refresh_para_ids_in_paragraph(paragraph, duplicates, used, seen);
        }
    }
}

fn refresh_para_ids_in_document_child(
    child: &mut DocumentChild,
    duplicates: &HashSet<String>,
    used: &mut HashSet<String>,
    seen: &mut HashSet<String>,
) {
    match child {
        DocumentChild::Paragraph(paragraph) => {
            refresh_para_ids_in_paragraph(paragraph, duplicates, used, seen)
        }
        DocumentChild::Table(table) => refresh_para_ids_in_table(table, duplicates, used, seen),
        DocumentChild::StructuredDataTag(tag) => {
            refresh_para_ids_in_structured_data_tag(tag, duplicates, used, seen)
        }
        DocumentChild::TableOfContents(toc) => refresh_para_ids_in_toc(toc, duplicates, used, seen),
        DocumentChild::Section(section) => {
            refresh_para_ids_in_section(section, duplicates, used, seen)
        }
        DocumentChild::CommentStart(c) => {
            refresh_para_ids_in_comment(&mut c.comment, duplicates, used, seen)
        }
        _ => {}
    }
}

fn refresh_para_ids_in_section(
    section: &mut Section,
    duplicates: &HashSet<String>,
    used: &mut HashSet<String>,
    seen: &mut HashSet<String>,
) {
    for child in &mut section.children {
        match child {
            SectionChild::Paragraph(paragraph) => {
                refresh_para_ids_in_paragraph(paragraph, duplicates, used, seen)
            }
            SectionChild::Table(table) => refresh_para_ids_in_table(table, duplicates, used, seen),
            SectionChild::StructuredDataTag(tag) => {
                refresh_para_ids_in_structured_data_tag(tag, duplicates, used, seen)
            }
            SectionChild::TableOfContents(toc) => {
                refresh_para_ids_in_toc(toc, duplicates, used, seen)
            }
            SectionChild::CommentStart(c) => {
                refresh_para_ids_in_comment(&mut c.comment, duplicates, used, seen)
            }
            _ => {}
        }
    }
    refresh_para_ids_in_section_property(&mut section.property, duplicates, used, seen);
}

fn refresh_para_ids_in_section_property(
    property: &mut SectionProperty,
    duplicates: &HashSet<String>,
    used: &mut HashSet<String>,
    seen: &mut HashSet<String>,
) {
    if let Some((_, header)) = property.header.as_mut() {
        refresh_para_ids_in_header(header, duplicates, used, seen);
    }
    if let Some((_, header)) = property.first_header.as_mut() {
        refresh_para_ids_in_header(header, duplicates, used, seen);
    }
    if let Some((_, header)) = property.even_header.as_mut() {
        refresh_para_ids_in_header(header, duplicates, used, seen);
    }
    if let Some((_, footer)) = property.footer.as_mut() {
        refresh_para_ids_in_footer(footer, duplicates, used, seen);
    }
    if let Some((_, footer)) = property.first_footer.as_mut() {
        refresh_para_ids_in_footer(footer, duplicates, used, seen);
    }
    if let Some((_, footer)) = property.even_footer.as_mut() {
        refresh_para_ids_in_footer(footer, duplicates, used, seen);
    }
}

fn refresh_para_ids_in_header(
    header: &mut Header,
    duplicates: &HashSet<String>,
    used: &mut HashSet<String>,
    seen: &mut HashSet<String>,
) {
    for child in &mut header.children {
        match child {
            HeaderChild::Paragraph(paragraph) => {
                refresh_para_ids_in_paragraph(paragraph, duplicates, used, seen)
            }
            HeaderChild::Table(table) => refresh_para_ids_in_table(table, duplicates, used, seen),
            HeaderChild::StructuredDataTag(tag) => {
                refresh_para_ids_in_structured_data_tag(tag, duplicates, used, seen)
            }
        }
    }
}

fn refresh_para_ids_in_footer(
    footer: &mut Footer,
    duplicates: &HashSet<String>,
    used: &mut HashSet<String>,
    seen: &mut HashSet<String>,
) {
    for child in &mut footer.children {
        match child {
            FooterChild::Paragraph(paragraph) => {
                refresh_para_ids_in_paragraph(paragraph, duplicates, used, seen)
            }
            FooterChild::Table(table) => refresh_para_ids_in_table(table, duplicates, used, seen),
            FooterChild::StructuredDataTag(tag) => {
                refresh_para_ids_in_structured_data_tag(tag, duplicates, used, seen)
            }
        }
    }
}

fn refresh_para_ids_in_toc(
    toc: &mut TableOfContents,
    duplicates: &HashSet<String>,
    used: &mut HashSet<String>,
    seen: &mut HashSet<String>,
) {
    for child in &mut toc.before_contents {
        match child {
            TocContent::Paragraph(paragraph) => {
                refresh_para_ids_in_paragraph(paragraph, duplicates, used, seen)
            }
            TocContent::Table(table) => refresh_para_ids_in_table(table, duplicates, used, seen),
        }
    }
    for child in &mut toc.after_contents {
        match child {
            TocContent::Paragraph(paragraph) => {
                refresh_para_ids_in_paragraph(paragraph, duplicates, used, seen)
            }
            TocContent::Table(table) => refresh_para_ids_in_table(table, duplicates, used, seen),
        }
    }
}

fn refresh_para_ids_in_table(
    table: &mut Table,
    duplicates: &HashSet<String>,
    used: &mut HashSet<String>,
    seen: &mut HashSet<String>,
) {
    for TableChild::TableRow(row) in &mut table.rows {
        for TableRowChild::TableCell(cell) in &mut row.cells {
            for content in &mut cell.children {
                match content {
                    TableCellContent::Paragraph(paragraph) => {
                        refresh_para_ids_in_paragraph(paragraph, duplicates, used, seen)
                    }
                    TableCellContent::Table(table) => {
                        refresh_para_ids_in_table(table, duplicates, used, seen)
                    }
                    TableCellContent::StructuredDataTag(tag) => {
                        refresh_para_ids_in_structured_data_tag(tag, duplicates, used, seen)
                    }
                    TableCellContent::TableOfContents(toc) => {
                        refresh_para_ids_in_toc(toc, duplicates, used, seen)
                    }
                }
            }
        }
    }
}

fn refresh_para_ids_in_paragraph(
    paragraph: &mut Paragraph,
    duplicates: &HashSet<String>,
    used: &mut HashSet<String>,
    seen: &mut HashSet<String>,
) {
    ensure_unique_paragraph_id(paragraph, duplicates, used, seen);

    for child in &mut paragraph.children {
        match child {
            ParagraphChild::Run(run) => refresh_para_ids_in_run(run, duplicates, used, seen),
            ParagraphChild::CommentStart(c) => {
                refresh_para_ids_in_comment(&mut c.comment, duplicates, used, seen)
            }
            ParagraphChild::Insert(insert) => {
                refresh_para_ids_in_insert(insert, duplicates, used, seen)
            }
            ParagraphChild::Delete(delete) => {
                refresh_para_ids_in_delete(delete, duplicates, used, seen)
            }
            ParagraphChild::MoveFrom(moved) => {
                refresh_para_ids_in_move_from(moved, duplicates, used, seen)
            }
            ParagraphChild::MoveTo(moved) => {
                refresh_para_ids_in_move_to(moved, duplicates, used, seen)
            }
            ParagraphChild::Hyperlink(hyperlink) => {
                refresh_para_ids_in_hyperlink(hyperlink, duplicates, used, seen)
            }
            ParagraphChild::StructuredDataTag(tag) => {
                refresh_para_ids_in_structured_data_tag(tag, duplicates, used, seen)
            }
            _ => {}
        }
    }
}

fn refresh_para_ids_in_hyperlink(
    hyperlink: &mut Hyperlink,
    duplicates: &HashSet<String>,
    used: &mut HashSet<String>,
    seen: &mut HashSet<String>,
) {
    for child in &mut hyperlink.children {
        match child {
            ParagraphChild::Run(run) => refresh_para_ids_in_run(run, duplicates, used, seen),
            ParagraphChild::CommentStart(c) => {
                refresh_para_ids_in_comment(&mut c.comment, duplicates, used, seen)
            }
            ParagraphChild::Insert(insert) => {
                refresh_para_ids_in_insert(insert, duplicates, used, seen)
            }
            ParagraphChild::Delete(delete) => {
                refresh_para_ids_in_delete(delete, duplicates, used, seen)
            }
            ParagraphChild::MoveFrom(moved) => {
                refresh_para_ids_in_move_from(moved, duplicates, used, seen)
            }
            ParagraphChild::MoveTo(moved) => {
                refresh_para_ids_in_move_to(moved, duplicates, used, seen)
            }
            ParagraphChild::Hyperlink(hyperlink) => {
                refresh_para_ids_in_hyperlink(hyperlink, duplicates, used, seen)
            }
            ParagraphChild::StructuredDataTag(tag) => {
                refresh_para_ids_in_structured_data_tag(tag, duplicates, used, seen)
            }
            _ => {}
        }
    }
}

fn refresh_para_ids_in_insert(
    insert: &mut Insert,
    duplicates: &HashSet<String>,
    used: &mut HashSet<String>,
    seen: &mut HashSet<String>,
) {
    for child in &mut insert.children {
        match child {
            InsertChild::Run(run) => refresh_para_ids_in_run(run, duplicates, used, seen),
            InsertChild::CommentStart(c) => {
                refresh_para_ids_in_comment(&mut c.comment, duplicates, used, seen)
            }
            InsertChild::Delete(delete) => {
                refresh_para_ids_in_delete(delete, duplicates, used, seen)
            }
            _ => {}
        }
    }
}

fn refresh_para_ids_in_delete(
    delete: &mut Delete,
    duplicates: &HashSet<String>,
    used: &mut HashSet<String>,
    seen: &mut HashSet<String>,
) {
    for child in &mut delete.children {
        match child {
            DeleteChild::Run(run) => refresh_para_ids_in_run(run, duplicates, used, seen),
            DeleteChild::CommentStart(c) => {
                refresh_para_ids_in_comment(&mut c.comment, duplicates, used, seen)
            }
            DeleteChild::CommentEnd(_) => {}
        }
    }
}

fn refresh_para_ids_in_move_from(
    moved: &mut MoveFrom,
    duplicates: &HashSet<String>,
    used: &mut HashSet<String>,
    seen: &mut HashSet<String>,
) {
    for child in &mut moved.children {
        match child {
            MoveFromChild::Run(run) => refresh_para_ids_in_run(run, duplicates, used, seen),
            MoveFromChild::CommentStart(c) => {
                refresh_para_ids_in_comment(&mut c.comment, duplicates, used, seen)
            }
            MoveFromChild::CommentEnd(_) => {}
        }
    }
}

fn refresh_para_ids_in_move_to(
    moved: &mut MoveTo,
    duplicates: &HashSet<String>,
    used: &mut HashSet<String>,
    seen: &mut HashSet<String>,
) {
    for child in &mut moved.children {
        match child {
            MoveToChild::Run(run) => refresh_para_ids_in_run(run, duplicates, used, seen),
            MoveToChild::Delete(delete) => {
                refresh_para_ids_in_delete(delete, duplicates, used, seen)
            }
            MoveToChild::CommentStart(c) => {
                refresh_para_ids_in_comment(&mut c.comment, duplicates, used, seen)
            }
            _ => {}
        }
    }
}

fn refresh_para_ids_in_run(
    run: &mut Run,
    duplicates: &HashSet<String>,
    used: &mut HashSet<String>,
    seen: &mut HashSet<String>,
) {
    for child in &mut run.children {
        if let RunChild::CommentStart(c) = child {
            refresh_para_ids_in_comment(&mut c.comment, duplicates, used, seen);
        }
    }
}

fn refresh_para_ids_in_structured_data_tag(
    tag: &mut StructuredDataTag,
    duplicates: &HashSet<String>,
    used: &mut HashSet<String>,
    seen: &mut HashSet<String>,
) {
    for child in &mut tag.children {
        match child {
            StructuredDataTagChild::Run(run) => {
                refresh_para_ids_in_run(run, duplicates, used, seen)
            }
            StructuredDataTagChild::Paragraph(paragraph) => {
                refresh_para_ids_in_paragraph(paragraph, duplicates, used, seen)
            }
            StructuredDataTagChild::Table(table) => {
                refresh_para_ids_in_table(table, duplicates, used, seen)
            }
            StructuredDataTagChild::CommentStart(c) => {
                refresh_para_ids_in_comment(&mut c.comment, duplicates, used, seen)
            }
            StructuredDataTagChild::StructuredDataTag(inner) => {
                refresh_para_ids_in_structured_data_tag(inner, duplicates, used, seen)
            }
            _ => {}
        }
    }
}

fn refresh_para_ids_in_comment(
    comment: &mut Comment,
    duplicates: &HashSet<String>,
    used: &mut HashSet<String>,
    seen: &mut HashSet<String>,
) {
    for child in &mut comment.children {
        match child {
            CommentChild::Paragraph(paragraph) => {
                refresh_para_ids_in_paragraph(paragraph, duplicates, used, seen)
            }
            CommentChild::Table(table) => refresh_para_ids_in_table(table, duplicates, used, seen),
        }
    }
}

fn ensure_unique_paragraph_id(
    paragraph: &mut Paragraph,
    duplicates: &HashSet<String>,
    used: &mut HashSet<String>,
    seen: &mut HashSet<String>,
) {
    if !paragraph.id.is_empty() && !duplicates.contains(&paragraph.id) {
        return;
    }
    if !paragraph.id.is_empty() && seen.insert(paragraph.id.clone()) {
        return;
    }

    let new_id = next_unique_paragraph_id(used);
    paragraph.id = new_id.clone();
    used.insert(new_id);
}

/// Returns a generated paragraph ID, falling back to a deterministic scan.
///
/// Starting beyond the set cardinality avoids repeatedly revisiting a dense
/// prefix of generated IDs after documents from multiple sources are merged.
/// Lower gaps are deliberately ignored because uniqueness, not compactness, is
/// the package invariant.
fn next_unique_paragraph_id(used: &HashSet<String>) -> String {
    let generated = crate::generate_para_id();
    if !used.contains(&generated) {
        return generated;
    }

    let first_candidate = used
        .len()
        .checked_add(1)
        .expect("the paragraph ID space should not be exhausted");
    (first_candidate..)
        .map(|value| format!("{value:08x}"))
        .find(|candidate| !used.contains(candidate))
        .expect("the paragraph ID space should not be exhausted")
}

fn push_comment_and_comment_extended(
    comments: &mut Vec<Comment>,
    comments_extended: &mut Vec<CommentExtended>,
    comment_map: &HashMap<usize, String>,
    c: &CommentRangeStart,
) {
    let comment = c.get_comment_ref();
    for child in &comment.children {
        if let CommentChild::Paragraph(child) = child {
            let para_id = child.id.clone();
            comments.push(comment.clone());
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

/// Populates an automatic TOC and bookmarks matching headings in place.
///
/// `document_children[toc_index]` is the temporary placeholder installed by
/// [`Docx::expand_auto_tocs`]. Reusing the input allocation preserves child
/// order without rebuilding the entire document vector.
fn update_document_by_toc(
    mut document_children: Vec<DocumentChild>,
    styles: &Styles,
    mut toc: TableOfContents,
    toc_index: usize,
) -> Vec<DocumentChild> {
    let heading_map = styles.create_heading_style_map();
    let mut items = vec![];

    if toc.instr.heading_styles_range.is_none() && !toc.instr.styles_with_levels.is_empty() {
        // INFO: if \t option set without heading styles ranges, Microsoft word does not show ToC items...
        document_children[toc_index] = DocumentChild::TableOfContents(Box::new(toc));
        return document_children;
    }

    let (min, max) = toc.instr.heading_styles_range.unwrap_or((0, 9));

    {
        let style_map: std::collections::HashMap<&str, usize> = toc
            .instr
            .styles_with_levels
            .iter()
            .map(|style| {
                let (style_id, level) = &style.0;
                (style_id.as_str(), *level)
            })
            .collect();

        for child in &mut document_children {
            if let DocumentChild::Paragraph(paragraph) = child {
                let heading_level = paragraph
                    .property
                    .style
                    .as_ref()
                    .and_then(|style| heading_map.get(&style.val))
                    .copied();
                if let Some(heading_level) = heading_level {
                    if min <= heading_level && max >= heading_level {
                        let toc_key = TocKey::generate();
                        items.push(
                            TableOfContentsItem::new()
                                .text(paragraph.raw_text())
                                .toc_key(&toc_key)
                                .level(heading_level),
                        );
                        paragraph.wrap_by_bookmark(generate_bookmark_id(), &toc_key);
                    }

                    if let Some((_min, _max)) = toc.instr.tc_field_level_range {
                        // TODO: check tc field
                    }
                }

                // Support \t option. Collect toc items if style id matched.
                let custom_level = paragraph
                    .property
                    .style
                    .as_ref()
                    .and_then(|style| style_map.get(style.val.as_str()))
                    .copied();
                if let Some(level) = custom_level {
                    if min <= level && max >= level {
                        let toc_key = TocKey::generate();
                        items.push(
                            TableOfContentsItem::new()
                                .text(paragraph.raw_text())
                                .toc_key(&toc_key)
                                .level(level),
                        );
                        paragraph.wrap_by_bookmark(generate_bookmark_id(), &toc_key);
                    }
                }
            }
        }
    }

    toc.items = items;
    document_children[toc_index] = DocumentChild::TableOfContents(Box::new(toc));
    document_children
}

#[cfg(test)]
mod toc_expansion_tests {
    use super::*;

    fn only_toc(docx: &Docx) -> &TableOfContents {
        let [DocumentChild::TableOfContents(toc)] = docx.document.children.as_slice() else {
            panic!("expected exactly one table of contents");
        };
        toc
    }

    #[test]
    fn static_toc_is_preserved_while_reporting_toc_presence() {
        let toc = TableOfContents::new()
            .alias("static")
            .add_before_paragraph(Paragraph::new().add_run(Run::new().add_text("before")));
        let expected = toc.clone();
        let mut docx = Docx::new().add_table_of_contents(toc);

        assert!(docx.expand_auto_tocs());
        assert_eq!(only_toc(&docx), &expected);
    }

    #[test]
    fn non_expandable_auto_toc_is_restored_after_being_moved() {
        let toc = TableOfContents::new()
            .alias("custom")
            .add_style_with_level(StyleWithLevel::new("Custom", 1))
            .auto();
        let expected = toc.clone();
        let mut docx = Docx::new().add_table_of_contents(toc);

        assert!(docx.expand_auto_tocs());
        assert_eq!(only_toc(&docx), &expected);
    }

    #[test]
    fn auto_toc_expansion_preserves_child_order_and_bookmarks_headings() {
        let mut docx = Docx::new()
            .add_style(Style::new("Heading1", crate::StyleType::Paragraph).name("Heading 1"))
            .add_paragraph(Paragraph::new().add_run(Run::new().add_text("before")))
            .add_table_of_contents(TableOfContents::new().heading_styles_range(1, 3).auto())
            .add_paragraph(
                Paragraph::new()
                    .style("Heading1")
                    .add_run(Run::new().add_text("Heading")),
            )
            .add_table(Table::new(vec![]));

        assert!(docx.expand_auto_tocs());
        assert!(matches!(
            docx.document.children.as_slice(),
            [
                DocumentChild::Paragraph(_),
                DocumentChild::TableOfContents(_),
                DocumentChild::Paragraph(_),
                DocumentChild::Table(_)
            ]
        ));

        let DocumentChild::TableOfContents(toc) = &docx.document.children[1] else {
            panic!("expected the table of contents to remain at its original index");
        };
        assert_eq!(toc.items.len(), 1);
        assert_eq!(toc.items[0].text, "Heading");

        let DocumentChild::Paragraph(heading) = &docx.document.children[2] else {
            panic!("expected the heading to remain at its original index");
        };
        assert!(matches!(
            heading.children.first(),
            Some(ParagraphChild::BookmarkStart(_))
        ));
        assert!(matches!(
            heading.children.last(),
            Some(ParagraphChild::BookmarkEnd(_))
        ));
    }

    #[test]
    fn auto_toc_expansion_matches_custom_styles_without_owning_their_ids() {
        let mut docx = Docx::new()
            .add_table_of_contents(
                TableOfContents::new()
                    .heading_styles_range(1, 9)
                    .add_style_with_level(StyleWithLevel::new("CustomHeading", 2))
                    .auto(),
            )
            .add_paragraph(
                Paragraph::new()
                    .style("CustomHeading")
                    .add_run(Run::new().add_text("Custom heading")),
            );

        assert!(docx.expand_auto_tocs());
        let DocumentChild::TableOfContents(toc) = &docx.document.children[0] else {
            panic!("expected an expanded table of contents");
        };
        assert_eq!(toc.items.len(), 1);
        assert_eq!(toc.items[0].text, "Custom heading");
        assert_eq!(toc.items[0].level, 2);
    }
}

#[cfg(test)]
mod image_collection_tests {
    use super::*;

    fn picture(id: &str, bytes: Vec<u8>) -> Pic {
        let mut picture = Pic::new_with_dimensions(bytes, 1, 1);
        picture.id = id.to_owned();
        picture
    }

    fn image_paragraph(id: &str, bytes: Vec<u8>) -> Paragraph {
        Paragraph::new().add_run(Run::new().add_image(picture(id, bytes)))
    }

    #[test]
    fn structured_footer_images_use_footer_relationship_ids() {
        let tag = StructuredDataTag::new().add_paragraph(
            Paragraph::new().add_run(Run::new().add_image(Pic::new_with_dimensions(
                vec![1, 2, 3],
                1,
                1,
            ))),
        );
        let footer = Footer::new().add_structured_data_tag(tag);
        let xml = Docx::new().footer(footer).build();

        assert!(String::from_utf8_lossy(&xml.footer_rels[0]).contains("Id=\"footer"));
        assert!(xml.media[0].0.starts_with("footer"));
    }

    #[test]
    fn repeated_header_images_keep_relationships_in_each_part() {
        let bytes = vec![1, 2, 3, 4];
        let xml = Docx::new()
            .header(Header::new().add_paragraph(image_paragraph("default", bytes.clone())))
            .first_header(Header::new().add_paragraph(image_paragraph("first", bytes)))
            .build();

        assert_eq!(xml.media.len(), 1);
        assert!(String::from_utf8_lossy(&xml.header_rels[0]).contains("relationships/image"));
        assert!(String::from_utf8_lossy(&xml.header_rels[1]).contains("relationships/image"));
    }

    #[test]
    fn repeated_images_share_one_media_entry_across_package_parts() {
        let bytes = vec![5, 6, 7, 8];
        let xml = Docx::new()
            .add_paragraph(image_paragraph("body", bytes.clone()))
            .header(Header::new().add_paragraph(image_paragraph("header", bytes.clone())))
            .footer(Footer::new().add_paragraph(image_paragraph("footer", bytes)))
            .build();

        assert_eq!(xml.media.len(), 1);
        assert!(String::from_utf8_lossy(&xml.document_rels).contains("relationships/image"));
        assert!(String::from_utf8_lossy(&xml.header_rels[0]).contains("relationships/image"));
        assert!(String::from_utf8_lossy(&xml.footer_rels[0]).contains("relationships/image"));
    }

    #[test]
    fn images_are_collected_from_top_level_sdt_and_section_content() {
        let tag = StructuredDataTag::new().add_paragraph(image_paragraph("sdt", vec![9]));
        let section = Section::new().add_paragraph(image_paragraph("section", vec![10]));
        let xml = Docx::new()
            .add_structured_data_tag(tag)
            .add_section(section)
            .build();

        assert_eq!(xml.media.len(), 2);
        assert_eq!(
            String::from_utf8_lossy(&xml.document_rels)
                .matches("relationships/image")
                .count(),
            2
        );
    }
}

#[cfg(test)]
mod document_traversal_tests {
    use super::*;

    #[test]
    fn footnotes_are_collected_from_table_paragraphs() {
        let footnote = Footnote::new()
            .add_content(Paragraph::new().add_run(Run::new().add_text("nested footnote")));
        let table = Table::new(vec![TableRow::new(vec![TableCell::new().add_paragraph(
            Paragraph::new().add_run(Run::new().add_footnote_reference(footnote)),
        )])]);

        let xml = Docx::new().add_table(table).build();

        assert!(String::from_utf8_lossy(&xml.footnotes).contains("nested footnote"));
        assert!(String::from_utf8_lossy(&xml.document_rels).contains("relationships/footnotes"));
    }
}

#[cfg(test)]
mod section_part_tests {
    use super::*;

    fn image_paragraph(picture: Pic) -> Paragraph {
        Paragraph::new().add_run(Run::new().add_image(picture))
    }

    #[test]
    fn add_section_preserves_all_header_and_footer_variants() {
        let section = Section::new()
            .header(Header::new())
            .first_header(Header::new())
            .even_header(Header::new())
            .footer(Footer::new())
            .first_footer(Footer::new())
            .even_footer(Footer::new());

        let docx = Docx::new().add_section(section);

        assert_eq!(docx.document_rels.header_count, 3);
        assert_eq!(docx.document_rels.footer_count, 3);
        let DocumentChild::Section(section) = &docx.document.children[0] else {
            panic!("expected a section");
        };
        assert_eq!(section.property.get_headers().len(), 3);
        assert_eq!(section.property.get_footers().len(), 3);

        let xml = docx.build();
        assert_eq!(xml.headers.len(), 3);
        assert_eq!(xml.footers.len(), 3);

        let content_types = String::from_utf8_lossy(&xml.content_type);
        for part_name in [
            "header1.xml",
            "header2.xml",
            "header3.xml",
            "footer1.xml",
            "footer2.xml",
            "footer3.xml",
        ] {
            assert!(
                content_types.contains(part_name),
                "missing content type for {part_name}"
            );
        }
    }

    #[test]
    fn section_images_create_relationships_for_every_part() {
        let picture = Pic::new_with_dimensions(vec![1, 2, 3, 4], 1, 1);
        let mut docx = Docx::new();

        for _ in 0..4 {
            docx = docx.add_section(
                Section::new()
                    .header(Header::new().add_paragraph(image_paragraph(picture.clone())))
                    .footer(Footer::new().add_paragraph(image_paragraph(picture.clone()))),
            );
        }

        let direct = docx.clone();
        let xml = docx.build();

        assert_eq!(xml.headers.len(), 4);
        assert_eq!(xml.header_rels.len(), 4);
        assert_eq!(xml.footers.len(), 4);
        assert_eq!(xml.footer_rels.len(), 4);
        assert!(xml
            .header_rels
            .iter()
            .all(|rels| String::from_utf8_lossy(rels).contains("relationships/image")));
        assert!(xml
            .footer_rels
            .iter()
            .all(|rels| String::from_utf8_lossy(rels).contains("relationships/image")));
        assert_eq!(xml.media.len(), 1);

        let mut cursor = std::io::Cursor::new(Vec::new());
        direct
            .pack(&mut cursor)
            .expect("failed to directly pack document");
        let mut archive =
            zip::ZipArchive::new(std::io::Cursor::new(cursor.into_inner())).expect("invalid ZIP");
        for path in ["word/_rels/header4.xml.rels", "word/_rels/footer4.xml.rels"] {
            let mut relationships = String::new();
            std::io::Read::read_to_string(
                &mut archive.by_name(path).expect("missing relationship part"),
                &mut relationships,
            )
            .expect("invalid relationship XML");
            assert!(relationships.contains("relationships/image"));
        }
    }

    #[test]
    fn section_parts_keep_relationship_creation_order_past_nine() {
        let mut docx = Docx::new();
        for number in 1..=12 {
            docx = docx.add_section(Section::new().header(Header::new().add_paragraph(
                Paragraph::new().add_run(Run::new().add_text(format!("header-{number}"))),
            )));
        }

        let xml = docx.build();

        assert_eq!(xml.headers.len(), 12);
        for (index, header) in xml.headers.iter().enumerate() {
            let expected = format!("header-{}", index + 1);
            assert!(
                String::from_utf8_lossy(header).contains(&expected),
                "header part {} did not contain {expected}",
                index + 1
            );
        }
    }

    #[test]
    fn section_parts_without_images_omit_relationship_files() {
        let xml = Docx::new()
            .add_section(Section::new().header(Header::new()).footer(Footer::new()))
            .build();

        assert!(xml.header_rels.is_empty());
        assert!(xml.footer_rels.is_empty());

        let mut cursor = std::io::Cursor::new(Vec::new());
        xml.pack(&mut cursor).expect("failed to pack document");
        let mut archive =
            zip::ZipArchive::new(std::io::Cursor::new(cursor.into_inner())).expect("invalid ZIP");
        assert!(archive.by_name("word/_rels/header1.xml.rels").is_err());
        assert!(archive.by_name("word/_rels/footer1.xml.rels").is_err());
    }

    #[test]
    fn sparse_section_relationships_keep_their_part_index() {
        let mut docx = Docx::new();
        for _ in 0..3 {
            docx = docx.add_section(Section::new().header(Header::new()));
        }
        docx = docx.add_section(Section::new().header(Header::new().add_paragraph(
            image_paragraph(Pic::new_with_dimensions(vec![1, 2, 3, 4], 1, 1)),
        )));

        let xml = docx.build();

        assert_eq!(xml.header_rels.len(), 4);
        assert!(xml.header_rels[..3].iter().all(Vec::is_empty));
        assert!(String::from_utf8_lossy(&xml.header_rels[3]).contains("relationships/image"));
    }
}

#[cfg(test)]
mod pack_tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn direct_pack_matches_buffered_package_output() {
        let image = vec![1, 2, 3, 4];
        let header = Header::new().add_paragraph(
            Paragraph::new().add_run(Run::new().add_image(Pic::new_with_dimensions(
                image.clone(),
                1,
                1,
            ))),
        );
        let footer = Footer::new().add_paragraph(
            Paragraph::new().add_run(Run::new().add_image(Pic::new_with_dimensions(image, 1, 1))),
        );
        let docx = Docx::new()
            .header(header)
            .footer(footer)
            .add_paragraph(Paragraph::new().add_run(Run::new().add_text("streamed package")));

        let mut buffered = Cursor::new(Vec::new());
        let xml = docx.clone().build();
        xml.pack(&mut buffered).unwrap();

        let mut streamed = Cursor::new(Vec::new());
        docx.pack(&mut streamed).unwrap();

        assert_eq!(streamed.into_inner(), buffered.into_inner());
    }
}

#[cfg(test)]
mod paragraph_id_tests {
    use super::*;

    fn paragraph_ids(docx: &Docx) -> Vec<&str> {
        docx.document
            .children
            .iter()
            .filter_map(|child| match child {
                DocumentChild::Paragraph(paragraph) => Some(paragraph.id.as_str()),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn unique_paragraph_ids_remain_unchanged() {
        let mut docx = Docx::new()
            .add_paragraph(Paragraph::new().id("first"))
            .add_paragraph(Paragraph::new().id("second"));

        docx.refresh_duplicate_para_ids();

        assert_eq!(paragraph_ids(&docx), ["first", "second"]);
    }

    #[test]
    fn duplicate_paragraph_ids_are_refreshed() {
        let mut docx = Docx::new()
            .add_paragraph(Paragraph::new().id("duplicate"))
            .add_paragraph(Paragraph::new().id("duplicate"));

        docx.refresh_duplicate_para_ids();

        let ids = paragraph_ids(&docx);
        assert_eq!(ids[0], "duplicate");
        assert_ne!(ids[1], "duplicate");
        assert_eq!(ids.iter().copied().collect::<HashSet<_>>().len(), 2);
    }

    #[test]
    fn empty_paragraph_id_is_refreshed() {
        let mut docx = Docx::new().add_paragraph(Paragraph::new().id(""));

        docx.refresh_duplicate_para_ids();

        assert!(!paragraph_ids(&docx)[0].is_empty());
    }

    #[test]
    fn duplicate_paragraph_ids_in_tracked_change_comments_are_refreshed() {
        let moved_from_comment = Comment::new(1).add_paragraph(Paragraph::new().id("duplicate"));
        let moved_to_comment = Comment::new(2).add_paragraph(Paragraph::new().id("duplicate"));
        let paragraph = Paragraph::new()
            .id("outer")
            .add_move_from(MoveFrom::new().add_comment_start(moved_from_comment))
            .add_move_to(MoveTo::new_with_empty().add_comment_start(moved_to_comment));
        let mut docx = Docx::new().add_paragraph(paragraph);

        docx.refresh_duplicate_para_ids();

        let mut counts = HashMap::new();
        collect_para_ids_in_docx(&docx, &mut counts);
        assert_eq!(counts.len(), 3);
        assert!(counts.values().all(|count| *count == 1));
    }

    #[test]
    fn auto_toc_paragraph_ids_are_normalized_after_expansion() {
        let heading = Paragraph::new()
            .id("00000001")
            .style("Heading1")
            .add_run(Run::new().add_text("Heading"));
        let mut docx = Docx::new()
            .add_style(Style::new("Heading1", crate::StyleType::Paragraph).name("Heading 1"))
            .add_table_of_contents(TableOfContents::new().heading_styles_range(1, 3).auto())
            .add_paragraph(heading);

        docx.prepare_for_build();

        let mut counts = HashMap::new();
        collect_para_ids_in_docx(&docx, &mut counts);
        assert!(!counts.contains_key(""));
        assert!(counts.values().all(|count| *count == 1));
    }

    #[test]
    fn paragraph_id_fallback_skips_the_dense_used_prefix() {
        let mut used: HashSet<String> = (1usize..=1_000)
            .map(|value| format!("{value:08x}"))
            .collect();
        used.insert(crate::generate_para_id());

        let first = next_unique_paragraph_id(&used);
        assert_eq!(first, "000003ea");
        used.insert(first);

        let second = next_unique_paragraph_id(&used);
        assert_eq!(second, "000003eb");
    }
}

#[cfg(test)]
mod emf_tests {
    use super::*;

    /// Build a syntactically-valid, minimal EMF: an EMR_HEADER followed by
    /// an EMR_EOF.
    pub(super) fn minimal_valid_emf() -> Vec<u8> {
        let mut buf = Vec::<u8>::with_capacity(108);

        // ---- EMR_HEADER (88 bytes) ----
        buf.extend_from_slice(&1u32.to_le_bytes()); // record type
        buf.extend_from_slice(&88u32.to_le_bytes()); // record size
                                                     // Bounds rect (RECTL)
        buf.extend_from_slice(&0i32.to_le_bytes());
        buf.extend_from_slice(&0i32.to_le_bytes());
        buf.extend_from_slice(&100i32.to_le_bytes());
        buf.extend_from_slice(&100i32.to_le_bytes());
        // Frame rect (RECTL)
        buf.extend_from_slice(&0i32.to_le_bytes());
        buf.extend_from_slice(&0i32.to_le_bytes());
        buf.extend_from_slice(&2540i32.to_le_bytes());
        buf.extend_from_slice(&2540i32.to_le_bytes());
        // " EMF" signature, version, total bytes, record count
        buf.extend_from_slice(&0x464D_4520u32.to_le_bytes());
        buf.extend_from_slice(&0x0001_0000u32.to_le_bytes());
        buf.extend_from_slice(&108u32.to_le_bytes());
        buf.extend_from_slice(&2u32.to_le_bytes());
        // handles, reserved, description, palette
        buf.extend_from_slice(&0u16.to_le_bytes());
        buf.extend_from_slice(&0u16.to_le_bytes());
        buf.extend_from_slice(&0u32.to_le_bytes());
        buf.extend_from_slice(&0u32.to_le_bytes());
        buf.extend_from_slice(&0u32.to_le_bytes());
        // Device size (SIZEL)
        buf.extend_from_slice(&1024i32.to_le_bytes());
        buf.extend_from_slice(&768i32.to_le_bytes());
        // Millimeters size (SIZEL)
        buf.extend_from_slice(&320i32.to_le_bytes());
        buf.extend_from_slice(&240i32.to_le_bytes());
        debug_assert_eq!(buf.len(), 88);

        // ---- EMR_EOF (20 bytes) ----
        buf.extend_from_slice(&14u32.to_le_bytes()); // record type
        buf.extend_from_slice(&20u32.to_le_bytes()); // record size
        buf.extend_from_slice(&0u32.to_le_bytes()); // nPalEntries
        buf.extend_from_slice(&0u32.to_le_bytes()); // offPalEntries
        buf.extend_from_slice(&20u32.to_le_bytes()); // SizeLast

        buf
    }

    /// 44-byte buffer with the right magic bytes but no payload. Detected
    /// as EMF, but can't actually be parsed — used to exercise the
    /// best-effort fallback.
    fn corrupt_emf_header() -> Vec<u8> {
        let mut buf = vec![0u8; 44];
        buf[0..4].copy_from_slice(&[0x01, 0x00, 0x00, 0x00]);
        buf[4..8].copy_from_slice(&[0x2C, 0x00, 0x00, 0x00]);
        buf[40..44].copy_from_slice(&[0x20, 0x45, 0x4D, 0x46]);
        buf
    }

    fn png_signature_bytes() -> Vec<u8> {
        // 8-byte PNG signature followed by zeros — enough to fail PNG
        // decoding but plenty to verify routing decisions.
        let mut buf = vec![137, 80, 78, 71, 13, 10, 26, 10];
        buf.resize(64, 0);
        buf
    }

    // ---------- is_emf ----------

    #[test]
    fn is_emf_detects_lowercase_extension() {
        assert!(is_emf("word/media/image1.emf", &[]));
    }

    #[test]
    fn is_emf_detects_uppercase_extension() {
        assert!(is_emf("word/media/IMAGE1.EMF", &[]));
    }

    #[test]
    fn is_emf_rejects_png_extension() {
        assert!(!is_emf("word/media/image1.png", &png_signature_bytes()));
    }

    #[test]
    fn is_emf_rejects_empty_buffer_with_unrelated_extension() {
        assert!(!is_emf("word/media/image1.bin", &[]));
    }

    #[test]
    fn is_emf_detects_magic_bytes_without_extension() {
        let buf = minimal_valid_emf();
        assert!(is_emf("word/media/image1.bin", &buf));
    }

    #[test]
    fn is_emf_rejects_buffer_too_short_for_signature() {
        // Has the record-type bytes but is shorter than 44 bytes, so the
        // signature check at offset 40 cannot pass.
        let buf = vec![0x01, 0x00, 0x00, 0x00];
        assert!(!is_emf("x.bin", &buf));
    }

    #[test]
    fn is_emf_rejects_wrong_signature() {
        // Right size, right record type, wrong signature bytes.
        let mut buf = vec![0u8; 44];
        buf[0..4].copy_from_slice(&[0x01, 0x00, 0x00, 0x00]);
        buf[40..44].copy_from_slice(&[0xDE, 0xAD, 0xBE, 0xEF]);
        assert!(!is_emf("x.bin", &buf));
    }

    // ---------- add_image routing ----------

    #[test]
    fn add_image_routes_valid_emf_into_images_with_empty_preview() {
        let buf = minimal_valid_emf();
        let docx = Docx::new().add_image("rId1", "word/media/image1.emf", buf.clone());

        assert_eq!(
            docx.images.len(),
            1,
            "EMF should land in the unified `images` vector"
        );
        let (id, path, original, preview) = &docx.images[0];
        assert_eq!(id, "rId1");
        assert_eq!(path, "word/media/image1.emf");
        assert_eq!(original.0, buf);
        assert!(preview.0.is_empty());
    }

    #[test]
    fn add_image_preserves_emf_detected_by_magic_bytes() {
        let buf = corrupt_emf_header();
        let docx = Docx::new().add_image("rId1", "word/media/image1.bin", buf.clone());
        assert_eq!(docx.images.len(), 1);
        let (_, _, original, preview) = &docx.images[0];
        assert_eq!(original.0, buf);
        assert!(preview.0.is_empty());
    }

    #[test]
    fn add_image_routes_png_as_png_preview() {
        let png_bytes = png_signature_bytes();
        let docx = Docx::new().add_image("rId1", "word/media/image1.png", png_bytes.clone());
        assert_eq!(docx.images.len(), 1);
        let (_, _, original, preview) = &docx.images[0];
        // PNG preview starts with the PNG signature, not `<svg`.
        assert_eq!(&preview.0[..8], &[137, 80, 78, 71, 13, 10, 26, 10]);
        assert_eq!(original.0, png_bytes);
        assert_eq!(preview.0, png_bytes);
    }

    // ---------- Docx JSON serialization ----------

    #[test]
    fn docx_serializes_emf_under_images_field() {
        let docx = Docx::new().add_image("rId1", "word/media/image1.emf", minimal_valid_emf());
        let json = serde_json::to_string(&docx).expect("should serialize");
        // EMF entries appear under the existing `images` key — no
        // separate `imagesEmf` key is emitted.
        assert!(json.contains("\"images\""));
        assert!(
            !json.contains("imagesEmf"),
            "EMF is unified into `images`; no separate `imagesEmf` key"
        );
        // The original bytes are base64-serialised inside the tuple.
        use base64::Engine;
        let b64 = base64::engine::general_purpose::STANDARD.encode(minimal_valid_emf());
        assert!(json.contains(&b64));
    }
}

/// Reader-level integration tests for EMF passthrough. We construct a tiny
/// in-memory docx (just enough relationships + a media entry) and run it
/// through `read_docx` to verify the original bytes surface on `Docx.images`.
#[cfg(test)]
mod emf_reader_tests {
    use super::emf_tests::minimal_valid_emf;
    use std::io::Write;

    /// Builds a minimum-viable docx ZIP that contains one EMF media
    /// file referenced from `document.xml.rels`. The XML payloads are
    /// the smallest accepted by the reader; the EMF is the same minimal
    /// header/EOF pair used in the unit tests.
    fn build_docx_with_emf() -> Vec<u8> {
        let buf = std::io::Cursor::new(Vec::<u8>::new());
        let mut zip = zip::ZipWriter::new(buf);
        let opts: zip::write::FileOptions<'_, ()> =
            zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

        let content_types = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
  <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
  <Default Extension="xml" ContentType="application/xml"/>
  <Default Extension="emf" ContentType="image/x-emf"/>
  <Override PartName="/word/document.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"/>
</Types>"#;
        let root_rels = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="word/document.xml"/>
</Relationships>"#;
        let doc_rels = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId10" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/image" Target="media/image1.emf"/>
</Relationships>"#;
        let document = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:body>
    <w:p><w:r><w:t>hello</w:t></w:r></w:p>
  </w:body>
</w:document>"#;

        for (path, body) in [
            ("[Content_Types].xml", content_types),
            ("_rels/.rels", root_rels),
            ("word/_rels/document.xml.rels", doc_rels),
            ("word/document.xml", document),
        ] {
            zip.start_file(path, opts).unwrap();
            zip.write_all(body.as_bytes()).unwrap();
        }
        zip.start_file("word/media/image1.emf", opts).unwrap();
        zip.write_all(&minimal_valid_emf()).unwrap();

        zip.finish().unwrap().into_inner()
    }

    #[test]
    fn read_docx_routes_emf_media_to_images_with_empty_preview() {
        let docx_bytes = build_docx_with_emf();
        let docx = crate::reader::read_docx(&docx_bytes).expect("should read docx");

        assert_eq!(
            docx.images.len(),
            1,
            "EMF media should be routed into the unified `images` vector"
        );
        let (id, path, original, preview) = &docx.images[0];
        assert_eq!(id, "rId10");
        assert!(path.ends_with("image1.emf"));
        assert_eq!(original.0, minimal_valid_emf());
        assert!(preview.0.is_empty());
    }
}
