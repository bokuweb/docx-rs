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
    collect_images_from_paragraph, collect_images_from_table, ImageDeduplicator,
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
        mut self,
        id: impl Into<String>,
        path: impl Into<String>,
        buf: Vec<u8>,
    ) -> Self {
        let path: String = path.into();

        if is_emf(&path, &buf) {
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

    pub fn add_section(mut self, s: Section) -> Docx {
        if s.has_numbering {
            // If this document has numbering, set numberings.xml to document_rels.
            // This is because numberings.xml without numbering cause an error on word online.
            self.document_rels.has_numberings = true;
        }

        let mut new_section = s;

        // header
        if let Some(header) = new_section.temp_header {
            if header.has_numbering {
                self.document_rels.has_numberings = true;
            }
            let count = self.document_rels.header_count + 1;
            new_section = Section {
                property: new_section
                    .property
                    .header(header, &create_header_rid(count)),
                children: new_section.children,
                has_numbering: new_section.has_numbering,
                temp_header: None,
                ..Default::default()
            };
            self.document_rels.header_count = count;
            self.content_type = self.content_type.add_header();
        }

        if let Some(header) = new_section.temp_first_header {
            if header.has_numbering {
                self.document_rels.has_numberings = true;
            }
            let count = self.document_rels.header_count + 1;
            new_section = Section {
                property: new_section
                    .property
                    .first_header(header, &create_header_rid(count)),
                children: new_section.children,
                has_numbering: new_section.has_numbering,
                temp_first_header: None,
                ..Default::default()
            };
            self.document_rels.header_count = count;
            self.content_type = self.content_type.add_header();
        }

        if let Some(header) = new_section.temp_even_header {
            if header.has_numbering {
                self.document_rels.has_numberings = true;
            }
            let count = self.document_rels.header_count + 1;
            new_section = Section {
                property: new_section
                    .property
                    .even_header(header, &create_header_rid(count)),
                children: new_section.children,
                has_numbering: new_section.has_numbering,
                temp_even_header: None,
                ..Default::default()
            };
            self.document_rels.header_count = count;
            self.content_type = self.content_type.add_header();
        }

        // header
        if let Some(header) = new_section.temp_header {
            if header.has_numbering {
                self.document_rels.has_numberings = true;
            }
            let count = self.document_rels.header_count + 1;
            new_section = Section {
                property: new_section
                    .property
                    .header(header, &create_header_rid(count)),
                children: new_section.children,
                has_numbering: new_section.has_numbering,
                temp_header: None,
                ..Default::default()
            };
            self.document_rels.header_count = count;
            self.content_type = self.content_type.add_header();
        }

        if let Some(header) = new_section.temp_first_header {
            if header.has_numbering {
                self.document_rels.has_numberings = true;
            }
            let count = self.document_rels.header_count + 1;
            new_section = Section {
                property: new_section
                    .property
                    .first_header(header, &create_header_rid(count)),
                children: new_section.children,
                has_numbering: new_section.has_numbering,
                temp_first_header: None,
                ..Default::default()
            };
            self.document_rels.header_count = count;
            self.content_type = self.content_type.add_header();
        }

        if let Some(header) = new_section.temp_even_header {
            if header.has_numbering {
                self.document_rels.has_numberings = true;
            }
            let count = self.document_rels.header_count + 1;
            new_section = Section {
                property: new_section
                    .property
                    .even_header(header, &create_header_rid(count)),
                children: new_section.children,
                has_numbering: new_section.has_numbering,
                temp_even_header: None,
                ..Default::default()
            };
            self.document_rels.header_count = count;
            self.content_type = self.content_type.add_header();
        }

        // footer
        if let Some(footer) = new_section.temp_footer {
            if footer.has_numbering {
                self.document_rels.has_numberings = true;
            }
            let count = self.document_rels.footer_count + 1;
            new_section = Section {
                property: new_section
                    .property
                    .footer(footer, &create_footer_rid(count)),
                children: new_section.children,
                has_numbering: new_section.has_numbering,
                temp_footer: None,
                ..Default::default()
            };
            self.document_rels.footer_count = count;
            self.content_type = self.content_type.add_footer();
        }

        if let Some(footer) = new_section.temp_first_footer {
            if footer.has_numbering {
                self.document_rels.has_numberings = true;
            }
            let count = self.document_rels.footer_count + 1;
            new_section = Section {
                property: new_section
                    .property
                    .first_footer(footer, &create_footer_rid(count)),
                children: new_section.children,
                has_numbering: new_section.has_numbering,
                temp_first_footer: None,
                ..Default::default()
            };
            self.document_rels.footer_count = count;
            self.content_type = self.content_type.add_footer();
        }

        if let Some(footer) = new_section.temp_even_footer {
            if footer.has_numbering {
                self.document_rels.has_numberings = true;
            }
            let count = self.document_rels.footer_count + 1;
            new_section = Section {
                property: new_section
                    .property
                    .even_footer(footer, &create_footer_rid(count)),
                children: new_section.children,
                has_numbering: new_section.has_numbering,
                temp_even_footer: None,
                ..Default::default()
            };
            self.document_rels.footer_count = count;
            self.content_type = self.content_type.add_footer();
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

    pub fn build(mut self) -> XMLDocx {
        self.reset();
        self.refresh_duplicate_para_ids();

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

        let has_toc = !tocs.is_empty();

        for (i, toc) in tocs {
            if toc.items.is_empty() && toc.auto {
                let children =
                    update_document_by_toc(self.document.children, &self.styles, *toc, i);
                self.document.children = children;
            }
        }

        let (images, mut images_bufs) = self.images_in_doc();
        let (header_images, header_images_bufs) = self.images_in_header();
        let (footer_images, footer_images_bufs) = self.images_in_footer();

        images_bufs.extend(header_images_bufs);
        images_bufs.extend(footer_images_bufs);

        let mut header_rels = vec![HeaderRels::new(); 3];
        for (i, images) in header_images.iter().enumerate() {
            if let Some(h) = header_rels.get_mut(i) {
                h.set_images(images.to_owned());
            }
        }
        let mut footer_rels = vec![FooterRels::new(); 3];
        for (i, images) in footer_images.iter().enumerate() {
            if let Some(f) = footer_rels.get_mut(i) {
                f.set_images(images.to_owned());
            }
        }

        let web_extensions = self.web_extensions.iter().map(|ext| ext.build()).collect();
        let custom_items = self.custom_items.iter().map(|xml| xml.build()).collect();
        let custom_item_props = self.custom_item_props.iter().map(|p| p.build()).collect();
        let custom_item_rels = self
            .custom_item_rels
            .iter()
            .map(|rel| rel.build())
            .collect();

        self.document_rels.images = images;

        let mut headers: Vec<&(String, Header)> = self.document.section_property.get_headers();

        self.document.children.iter().for_each(|child| {
            if let DocumentChild::Section(section) = child {
                for h in section.property.get_headers() {
                    headers.push(h);
                }
            }
        });

        headers.sort_by(|a, b| a.0.cmp(&b.0));
        let headers = headers.iter().map(|h| h.1.build()).collect();

        let mut footers: Vec<&(String, Footer)> = self.document.section_property.get_footers();

        self.document.children.iter().for_each(|child| {
            if let DocumentChild::Section(section) = child {
                for h in section.property.get_footers() {
                    footers.push(h);
                }
            }
        });

        footers.sort_by(|a, b| a.0.cmp(&b.0));
        let footers = footers.iter().map(|h| h.1.build()).collect();

        // Collect footnotes
        if self.collect_footnotes() {
            // Relationship entry for footnotes
            self.content_type = self.content_type.add_footnotes();
            self.document_rels.has_footnotes = true;
        }

        if has_toc {
            for i in 1..=9 {
                if !self
                    .styles
                    .styles
                    .iter()
                    .any(|s| s.name == Name::new(format!("toc {i}")))
                {
                    self.styles = self
                        .styles
                        .add_style(crate::documents::preset_styles::toc(i));
                }
            }
        }

        XMLDocx {
            content_type: self.content_type.build(),
            rels: self.rels.build(),
            doc_props: self.doc_props.build(),
            styles: self.styles.build(),
            document: self.document.build(),
            comments: self.comments.build(),
            document_rels: self.document_rels.build(),
            header_rels: header_rels.into_iter().map(|r| r.build()).collect(),
            footer_rels: footer_rels.into_iter().map(|r| r.build()).collect(),
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
            footnotes: self.footnotes.build(),
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

    // Internal: for docx-wasm
    pub fn comments_json(&mut self) -> String {
        self.reset();
        self.update_dependencies();
        serde_json::to_string_pretty(&self.comments).unwrap()
    }

    fn reset(&self) {
        crate::reset_para_id();
    }

    fn refresh_duplicate_para_ids(&mut self) {
        let mut counts: HashMap<String, usize> = HashMap::new();
        collect_para_ids_in_docx(self, &mut counts);

        let mut used: HashSet<String> = counts.keys().cloned().collect();
        let mut seen: HashSet<String> = HashSet::new();

        refresh_para_ids_in_docx(self, &counts, &mut used, &mut seen);
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
                            for child in &h.children {
                                if let ParagraphChild::CommentStart(c) = child {
                                    self.insert_comment_to_map(&mut comment_map, c);
                                }
                            }
                        }
                    }
                }
                DocumentChild::Table(table) => {
                    collect_comment_map_in_table(table, &mut comment_map);
                }
                DocumentChild::TableOfContents(toc) => {
                    for child in &toc.before_contents {
                        if let TocContent::Paragraph(paragraph) = child {
                            collect_comment_map_in_paragraph(paragraph, &mut comment_map);
                        }
                        if let TocContent::Table(table) = child {
                            collect_comment_map_in_table(table, &mut comment_map);
                        }
                    }
                    for child in &toc.after_contents {
                        if let TocContent::Paragraph(paragraph) = child {
                            collect_comment_map_in_paragraph(paragraph, &mut comment_map);
                        }
                        if let TocContent::Table(table) = child {
                            collect_comment_map_in_table(table, &mut comment_map);
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
                                } else if let InsertChild::Delete(ref mut d) = child {
                                    for child in &mut d.children {
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
    fn images_in_doc(&mut self) -> (Vec<ImageIdAndPath>, Vec<ImageIdAndBuf>) {
        let mut images: Vec<(String, String)> = vec![];
        let mut image_bufs: Vec<(String, Vec<u8>)> = vec![];
        let mut deduplicator = ImageDeduplicator::default();

        for child in &mut self.document.children {
            match child {
                DocumentChild::Paragraph(paragraph) => {
                    collect_images_from_paragraph(
                        paragraph,
                        &mut images,
                        &mut image_bufs,
                        None,
                        &mut deduplicator,
                    );
                }
                DocumentChild::Table(table) => {
                    collect_images_from_table(
                        table,
                        &mut images,
                        &mut image_bufs,
                        None,
                        &mut deduplicator,
                    );
                }
                _ => {}
            }
        }
        (images, image_bufs)
    }

    fn images_in_header(&mut self) -> (Vec<Vec<ImageIdAndPath>>, Vec<ImageIdAndBuf>) {
        let mut header_images: Vec<Vec<ImageIdAndPath>> = vec![vec![]; 3];
        let mut image_bufs: Vec<(String, Vec<u8>)> = vec![];
        let mut deduplicator = ImageDeduplicator::default();

        if let Some((_, header)) = &mut self.document.section_property.header.as_mut() {
            let mut images: Vec<ImageIdAndPath> = vec![];
            for child in header.children.iter_mut() {
                match child {
                    HeaderChild::Paragraph(paragraph) => {
                        collect_images_from_paragraph(
                            paragraph,
                            &mut images,
                            &mut image_bufs,
                            Some("header"),
                            &mut deduplicator,
                        );
                    }
                    HeaderChild::Table(table) => {
                        collect_images_from_table(
                            table,
                            &mut images,
                            &mut image_bufs,
                            Some("header"),
                            &mut deduplicator,
                        );
                    }
                    HeaderChild::StructuredDataTag(tag) => {
                        for child in tag.children.iter_mut() {
                            if let StructuredDataTagChild::Paragraph(paragraph) = child {
                                collect_images_from_paragraph(
                                    paragraph,
                                    &mut images,
                                    &mut image_bufs,
                                    Some("header"),
                                    &mut deduplicator,
                                );
                            }
                            if let StructuredDataTagChild::Table(table) = child {
                                collect_images_from_table(
                                    table,
                                    &mut images,
                                    &mut image_bufs,
                                    Some("header"),
                                    &mut deduplicator,
                                );
                            }
                        }
                    }
                }
            }
            header_images[0] = images;
        }

        if let Some((_, header)) = &mut self.document.section_property.first_header.as_mut() {
            let mut images: Vec<ImageIdAndPath> = vec![];
            for child in header.children.iter_mut() {
                match child {
                    HeaderChild::Paragraph(paragraph) => {
                        collect_images_from_paragraph(
                            paragraph,
                            &mut images,
                            &mut image_bufs,
                            Some("header"),
                            &mut deduplicator,
                        );
                    }
                    HeaderChild::Table(table) => {
                        collect_images_from_table(
                            table,
                            &mut images,
                            &mut image_bufs,
                            Some("header"),
                            &mut deduplicator,
                        );
                    }
                    HeaderChild::StructuredDataTag(tag) => {
                        for child in tag.children.iter_mut() {
                            if let StructuredDataTagChild::Paragraph(paragraph) = child {
                                collect_images_from_paragraph(
                                    paragraph,
                                    &mut images,
                                    &mut image_bufs,
                                    Some("header"),
                                    &mut deduplicator,
                                );
                            }
                            if let StructuredDataTagChild::Table(table) = child {
                                collect_images_from_table(
                                    table,
                                    &mut images,
                                    &mut image_bufs,
                                    Some("header"),
                                    &mut deduplicator,
                                );
                            }
                        }
                    }
                }
            }
            header_images[1] = images;
        }

        if let Some((_, header)) = &mut self.document.section_property.even_header.as_mut() {
            let mut images: Vec<ImageIdAndPath> = vec![];
            for child in header.children.iter_mut() {
                match child {
                    HeaderChild::Paragraph(paragraph) => {
                        collect_images_from_paragraph(
                            paragraph,
                            &mut images,
                            &mut image_bufs,
                            Some("header"),
                            &mut deduplicator,
                        );
                    }
                    HeaderChild::Table(table) => {
                        collect_images_from_table(
                            table,
                            &mut images,
                            &mut image_bufs,
                            Some("header"),
                            &mut deduplicator,
                        );
                    }
                    HeaderChild::StructuredDataTag(tag) => {
                        for child in tag.children.iter_mut() {
                            if let StructuredDataTagChild::Paragraph(paragraph) = child {
                                collect_images_from_paragraph(
                                    paragraph,
                                    &mut images,
                                    &mut image_bufs,
                                    Some("header"),
                                    &mut deduplicator,
                                );
                            }
                            if let StructuredDataTagChild::Table(table) = child {
                                collect_images_from_table(
                                    table,
                                    &mut images,
                                    &mut image_bufs,
                                    Some("header"),
                                    &mut deduplicator,
                                );
                            }
                        }
                    }
                }
            }
            header_images[2] = images;
        }
        (header_images, image_bufs)
    }

    // Traverse and collect images from header.
    fn images_in_footer(&mut self) -> (Vec<Vec<ImageIdAndPath>>, Vec<ImageIdAndBuf>) {
        let mut footer_images: Vec<Vec<ImageIdAndPath>> = vec![vec![]; 3];
        let mut image_bufs: Vec<(String, Vec<u8>)> = vec![];
        let mut deduplicator = ImageDeduplicator::default();

        if let Some((_, footer)) = &mut self.document.section_property.footer.as_mut() {
            let mut images: Vec<ImageIdAndPath> = vec![];
            for child in footer.children.iter_mut() {
                match child {
                    FooterChild::Paragraph(paragraph) => {
                        collect_images_from_paragraph(
                            paragraph,
                            &mut images,
                            &mut image_bufs,
                            Some("footer"),
                            &mut deduplicator,
                        );
                    }
                    FooterChild::Table(table) => {
                        collect_images_from_table(
                            table,
                            &mut images,
                            &mut image_bufs,
                            Some("footer"),
                            &mut deduplicator,
                        );
                    }
                    FooterChild::StructuredDataTag(tag) => {
                        for child in tag.children.iter_mut() {
                            if let StructuredDataTagChild::Paragraph(paragraph) = child {
                                collect_images_from_paragraph(
                                    paragraph,
                                    &mut images,
                                    &mut image_bufs,
                                    Some("header"),
                                    &mut deduplicator,
                                );
                            }
                            if let StructuredDataTagChild::Table(table) = child {
                                collect_images_from_table(
                                    table,
                                    &mut images,
                                    &mut image_bufs,
                                    Some("header"),
                                    &mut deduplicator,
                                );
                            }
                        }
                    }
                }
            }
            footer_images[0] = images;
        }

        if let Some((_, footer)) = &mut self.document.section_property.first_footer.as_mut() {
            let mut images: Vec<ImageIdAndPath> = vec![];
            for child in footer.children.iter_mut() {
                match child {
                    FooterChild::Paragraph(paragraph) => {
                        collect_images_from_paragraph(
                            paragraph,
                            &mut images,
                            &mut image_bufs,
                            Some("footer"),
                            &mut deduplicator,
                        );
                    }
                    FooterChild::Table(table) => {
                        collect_images_from_table(
                            table,
                            &mut images,
                            &mut image_bufs,
                            Some("footer"),
                            &mut deduplicator,
                        );
                    }
                    FooterChild::StructuredDataTag(tag) => {
                        for child in tag.children.iter_mut() {
                            if let StructuredDataTagChild::Paragraph(paragraph) = child {
                                collect_images_from_paragraph(
                                    paragraph,
                                    &mut images,
                                    &mut image_bufs,
                                    Some("header"),
                                    &mut deduplicator,
                                );
                            }
                            if let StructuredDataTagChild::Table(table) = child {
                                collect_images_from_table(
                                    table,
                                    &mut images,
                                    &mut image_bufs,
                                    Some("header"),
                                    &mut deduplicator,
                                );
                            }
                        }
                    }
                }
            }
            footer_images[1] = images;
        }

        if let Some((_, footer)) = &mut self.document.section_property.even_footer.as_mut() {
            let mut images: Vec<ImageIdAndPath> = vec![];
            for child in footer.children.iter_mut() {
                match child {
                    FooterChild::Paragraph(paragraph) => {
                        collect_images_from_paragraph(
                            paragraph,
                            &mut images,
                            &mut image_bufs,
                            Some("footer"),
                            &mut deduplicator,
                        );
                    }
                    FooterChild::Table(table) => {
                        collect_images_from_table(
                            table,
                            &mut images,
                            &mut image_bufs,
                            Some("footer"),
                            &mut deduplicator,
                        );
                    }
                    FooterChild::StructuredDataTag(tag) => {
                        for child in tag.children.iter_mut() {
                            if let StructuredDataTagChild::Paragraph(paragraph) = child {
                                collect_images_from_paragraph(
                                    paragraph,
                                    &mut images,
                                    &mut image_bufs,
                                    Some("header"),
                                    &mut deduplicator,
                                );
                            }
                            if let StructuredDataTagChild::Table(table) = child {
                                collect_images_from_table(
                                    table,
                                    &mut images,
                                    &mut image_bufs,
                                    Some("header"),
                                    &mut deduplicator,
                                );
                            }
                        }
                    }
                }
            }
            footer_images[2] = images;
        }
        (footer_images, image_bufs)
    }

    /// Collect footnotes from all Runs to the docx footnotes node.
    pub fn collect_footnotes(&mut self) -> bool {
        let footnotes: Vec<Footnote> = self
            .document
            .children
            .iter()
            .filter_map(|child| match child {
                DocumentChild::Paragraph(paragraph) => Some(&paragraph.children),
                _ => None,
            })
            .flat_map(|children| children.iter())
            .filter_map(|para_child| match para_child {
                ParagraphChild::Run(run) => Some(&run.children),
                _ => None,
            })
            .flat_map(|children| children.iter())
            .filter_map(|run_child| match run_child {
                RunChild::FootnoteReference(footnote_ref) => Some(footnote_ref),
                _ => None,
            })
            .map(Into::<Footnote>::into)
            .collect();
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

fn collect_comment_map_in_table(table: &Table, comment_map: &mut HashMap<usize, String>) {
    for TableChild::TableRow(row) in &table.rows {
        for TableRowChild::TableCell(cell) in &row.cells {
            for content in &cell.children {
                match content {
                    TableCellContent::Paragraph(paragraph) => {
                        collect_comment_map_in_paragraph(paragraph, comment_map);
                    }
                    TableCellContent::Table(table) => {
                        collect_comment_map_in_table(table, comment_map)
                    }
                    TableCellContent::StructuredDataTag(tag) => {
                        for child in &tag.children {
                            if let StructuredDataTagChild::Paragraph(paragraph) = child {
                                collect_comment_map_in_paragraph(paragraph, comment_map);
                            }
                            if let StructuredDataTagChild::Table(table) = child {
                                collect_comment_map_in_table(table, comment_map);
                            }
                        }
                    }
                    TableCellContent::TableOfContents(t) => {
                        for child in &t.before_contents {
                            if let TocContent::Paragraph(paragraph) = child {
                                collect_comment_map_in_paragraph(paragraph, comment_map);
                            }
                            if let TocContent::Table(table) = child {
                                collect_comment_map_in_table(table, comment_map);
                            }
                        }
                        for child in &t.after_contents {
                            if let TocContent::Paragraph(paragraph) = child {
                                collect_comment_map_in_paragraph(paragraph, comment_map);
                            }
                            if let TocContent::Table(table) = child {
                                collect_comment_map_in_table(table, comment_map);
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

fn store_comments_in_paragraph(paragraph: &mut Paragraph, comments: &[Comment]) {
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
                    if let Some(comment) = comments.iter().find(|c| c.id() == comment_id) {
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
                    if let Some(comment) = comments.iter().find(|c| c.id() == comment_id) {
                        let comment = comment.clone();
                        c.as_mut().comment(comment);
                    }
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
                        store_comments_in_paragraph(paragraph, comments)
                    }
                    TableCellContent::Table(ref mut table) => {
                        store_comments_in_table(table, comments);
                    }
                    TableCellContent::StructuredDataTag(ref mut tag) => {
                        for child in &mut tag.children {
                            if let StructuredDataTagChild::Paragraph(paragraph) = child {
                                store_comments_in_paragraph(paragraph, comments);
                            }
                            if let StructuredDataTagChild::Table(table) = child {
                                store_comments_in_table(table, comments);
                            }
                        }
                    }
                    TableCellContent::TableOfContents(ref mut t) => {
                        for child in &mut t.before_contents {
                            if let TocContent::Paragraph(paragraph) = child {
                                store_comments_in_paragraph(paragraph, comments);
                            }
                            if let TocContent::Table(table) = child {
                                store_comments_in_table(table, comments);
                            }
                        }

                        for child in &mut t.after_contents {
                            if let TocContent::Paragraph(paragraph) = child {
                                store_comments_in_paragraph(paragraph, comments);
                            }
                            if let TocContent::Table(table) = child {
                                store_comments_in_table(table, comments);
                            }
                        }
                    }
                }
            }
        }
    }
}

fn collect_para_ids_in_docx(docx: &Docx, counts: &mut HashMap<String, usize>) {
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

fn collect_para_ids_in_document_child(child: &DocumentChild, counts: &mut HashMap<String, usize>) {
    match child {
        DocumentChild::Paragraph(paragraph) => collect_para_ids_in_paragraph(paragraph, counts),
        DocumentChild::Table(table) => collect_para_ids_in_table(table, counts),
        DocumentChild::StructuredDataTag(tag) => {
            collect_para_ids_in_structured_data_tag(tag, counts)
        }
        DocumentChild::TableOfContents(toc) => collect_para_ids_in_toc(toc, counts),
        DocumentChild::Section(section) => collect_para_ids_in_section(section, counts),
        _ => {}
    }
}

fn collect_para_ids_in_section(section: &Section, counts: &mut HashMap<String, usize>) {
    for child in &section.children {
        match child {
            SectionChild::Paragraph(paragraph) => collect_para_ids_in_paragraph(paragraph, counts),
            SectionChild::Table(table) => collect_para_ids_in_table(table, counts),
            SectionChild::StructuredDataTag(tag) => {
                collect_para_ids_in_structured_data_tag(tag, counts)
            }
            SectionChild::TableOfContents(toc) => collect_para_ids_in_toc(toc, counts),
            _ => {}
        }
    }
    collect_para_ids_in_section_property(&section.property, counts);
}

fn collect_para_ids_in_section_property(
    property: &SectionProperty,
    counts: &mut HashMap<String, usize>,
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

fn collect_para_ids_in_header(header: &Header, counts: &mut HashMap<String, usize>) {
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

fn collect_para_ids_in_footer(footer: &Footer, counts: &mut HashMap<String, usize>) {
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

fn collect_para_ids_in_toc(toc: &TableOfContents, counts: &mut HashMap<String, usize>) {
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

fn collect_para_ids_in_table(table: &Table, counts: &mut HashMap<String, usize>) {
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

fn collect_para_ids_in_paragraph(paragraph: &Paragraph, counts: &mut HashMap<String, usize>) {
    *counts.entry(paragraph.id.clone()).or_insert(0) += 1;

    for child in &paragraph.children {
        match child {
            ParagraphChild::CommentStart(c) => collect_para_ids_in_comment(&c.comment, counts),
            ParagraphChild::Insert(insert) => collect_para_ids_in_insert(insert, counts),
            ParagraphChild::Delete(delete) => collect_para_ids_in_delete(delete, counts),
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

fn collect_para_ids_in_hyperlink(hyperlink: &Hyperlink, counts: &mut HashMap<String, usize>) {
    for child in &hyperlink.children {
        match child {
            ParagraphChild::CommentStart(c) => collect_para_ids_in_comment(&c.comment, counts),
            ParagraphChild::Insert(insert) => collect_para_ids_in_insert(insert, counts),
            ParagraphChild::Delete(delete) => collect_para_ids_in_delete(delete, counts),
            ParagraphChild::StructuredDataTag(tag) => {
                collect_para_ids_in_structured_data_tag(tag, counts)
            }
            _ => {}
        }
    }
}

fn collect_para_ids_in_insert(insert: &Insert, counts: &mut HashMap<String, usize>) {
    for child in &insert.children {
        match child {
            InsertChild::CommentStart(c) => collect_para_ids_in_comment(&c.comment, counts),
            InsertChild::Delete(delete) => collect_para_ids_in_delete(delete, counts),
            _ => {}
        }
    }
}

fn collect_para_ids_in_delete(delete: &Delete, counts: &mut HashMap<String, usize>) {
    for child in &delete.children {
        if let DeleteChild::CommentStart(c) = child {
            collect_para_ids_in_comment(&c.comment, counts);
        }
    }
}

fn collect_para_ids_in_structured_data_tag(
    tag: &StructuredDataTag,
    counts: &mut HashMap<String, usize>,
) {
    for child in &tag.children {
        match child {
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

fn collect_para_ids_in_comment(comment: &Comment, counts: &mut HashMap<String, usize>) {
    for child in &comment.children {
        match child {
            CommentChild::Paragraph(paragraph) => collect_para_ids_in_paragraph(paragraph, counts),
            CommentChild::Table(table) => collect_para_ids_in_table(table, counts),
        }
    }
}

fn refresh_para_ids_in_docx(
    docx: &mut Docx,
    counts: &HashMap<String, usize>,
    used: &mut HashSet<String>,
    seen: &mut HashSet<String>,
) {
    for child in &mut docx.document.children {
        refresh_para_ids_in_document_child(child, counts, used, seen);
    }
    refresh_para_ids_in_section_property(&mut docx.document.section_property, counts, used, seen);

    for comment in &mut docx.comments.comments {
        refresh_para_ids_in_comment(comment, counts, used, seen);
    }

    for footnote in &mut docx.footnotes.footnotes {
        for paragraph in &mut footnote.content {
            refresh_para_ids_in_paragraph(paragraph, counts, used, seen);
        }
    }
}

fn refresh_para_ids_in_document_child(
    child: &mut DocumentChild,
    counts: &HashMap<String, usize>,
    used: &mut HashSet<String>,
    seen: &mut HashSet<String>,
) {
    match child {
        DocumentChild::Paragraph(paragraph) => {
            refresh_para_ids_in_paragraph(paragraph, counts, used, seen)
        }
        DocumentChild::Table(table) => refresh_para_ids_in_table(table, counts, used, seen),
        DocumentChild::StructuredDataTag(tag) => {
            refresh_para_ids_in_structured_data_tag(tag, counts, used, seen)
        }
        DocumentChild::TableOfContents(toc) => refresh_para_ids_in_toc(toc, counts, used, seen),
        DocumentChild::Section(section) => refresh_para_ids_in_section(section, counts, used, seen),
        _ => {}
    }
}

fn refresh_para_ids_in_section(
    section: &mut Section,
    counts: &HashMap<String, usize>,
    used: &mut HashSet<String>,
    seen: &mut HashSet<String>,
) {
    for child in &mut section.children {
        match child {
            SectionChild::Paragraph(paragraph) => {
                refresh_para_ids_in_paragraph(paragraph, counts, used, seen)
            }
            SectionChild::Table(table) => refresh_para_ids_in_table(table, counts, used, seen),
            SectionChild::StructuredDataTag(tag) => {
                refresh_para_ids_in_structured_data_tag(tag, counts, used, seen)
            }
            SectionChild::TableOfContents(toc) => refresh_para_ids_in_toc(toc, counts, used, seen),
            _ => {}
        }
    }
    refresh_para_ids_in_section_property(&mut section.property, counts, used, seen);
}

fn refresh_para_ids_in_section_property(
    property: &mut SectionProperty,
    counts: &HashMap<String, usize>,
    used: &mut HashSet<String>,
    seen: &mut HashSet<String>,
) {
    if let Some((_, header)) = property.header.as_mut() {
        refresh_para_ids_in_header(header, counts, used, seen);
    }
    if let Some((_, header)) = property.first_header.as_mut() {
        refresh_para_ids_in_header(header, counts, used, seen);
    }
    if let Some((_, header)) = property.even_header.as_mut() {
        refresh_para_ids_in_header(header, counts, used, seen);
    }
    if let Some((_, footer)) = property.footer.as_mut() {
        refresh_para_ids_in_footer(footer, counts, used, seen);
    }
    if let Some((_, footer)) = property.first_footer.as_mut() {
        refresh_para_ids_in_footer(footer, counts, used, seen);
    }
    if let Some((_, footer)) = property.even_footer.as_mut() {
        refresh_para_ids_in_footer(footer, counts, used, seen);
    }
}

fn refresh_para_ids_in_header(
    header: &mut Header,
    counts: &HashMap<String, usize>,
    used: &mut HashSet<String>,
    seen: &mut HashSet<String>,
) {
    for child in &mut header.children {
        match child {
            HeaderChild::Paragraph(paragraph) => {
                refresh_para_ids_in_paragraph(paragraph, counts, used, seen)
            }
            HeaderChild::Table(table) => refresh_para_ids_in_table(table, counts, used, seen),
            HeaderChild::StructuredDataTag(tag) => {
                refresh_para_ids_in_structured_data_tag(tag, counts, used, seen)
            }
        }
    }
}

fn refresh_para_ids_in_footer(
    footer: &mut Footer,
    counts: &HashMap<String, usize>,
    used: &mut HashSet<String>,
    seen: &mut HashSet<String>,
) {
    for child in &mut footer.children {
        match child {
            FooterChild::Paragraph(paragraph) => {
                refresh_para_ids_in_paragraph(paragraph, counts, used, seen)
            }
            FooterChild::Table(table) => refresh_para_ids_in_table(table, counts, used, seen),
            FooterChild::StructuredDataTag(tag) => {
                refresh_para_ids_in_structured_data_tag(tag, counts, used, seen)
            }
        }
    }
}

fn refresh_para_ids_in_toc(
    toc: &mut TableOfContents,
    counts: &HashMap<String, usize>,
    used: &mut HashSet<String>,
    seen: &mut HashSet<String>,
) {
    for child in &mut toc.before_contents {
        match child {
            TocContent::Paragraph(paragraph) => {
                refresh_para_ids_in_paragraph(paragraph, counts, used, seen)
            }
            TocContent::Table(table) => refresh_para_ids_in_table(table, counts, used, seen),
        }
    }
    for child in &mut toc.after_contents {
        match child {
            TocContent::Paragraph(paragraph) => {
                refresh_para_ids_in_paragraph(paragraph, counts, used, seen)
            }
            TocContent::Table(table) => refresh_para_ids_in_table(table, counts, used, seen),
        }
    }
}

fn refresh_para_ids_in_table(
    table: &mut Table,
    counts: &HashMap<String, usize>,
    used: &mut HashSet<String>,
    seen: &mut HashSet<String>,
) {
    for TableChild::TableRow(row) in &mut table.rows {
        for TableRowChild::TableCell(cell) in &mut row.cells {
            for content in &mut cell.children {
                match content {
                    TableCellContent::Paragraph(paragraph) => {
                        refresh_para_ids_in_paragraph(paragraph, counts, used, seen)
                    }
                    TableCellContent::Table(table) => {
                        refresh_para_ids_in_table(table, counts, used, seen)
                    }
                    TableCellContent::StructuredDataTag(tag) => {
                        refresh_para_ids_in_structured_data_tag(tag, counts, used, seen)
                    }
                    TableCellContent::TableOfContents(toc) => {
                        refresh_para_ids_in_toc(toc, counts, used, seen)
                    }
                }
            }
        }
    }
}

fn refresh_para_ids_in_paragraph(
    paragraph: &mut Paragraph,
    counts: &HashMap<String, usize>,
    used: &mut HashSet<String>,
    seen: &mut HashSet<String>,
) {
    ensure_unique_paragraph_id(paragraph, counts, used, seen);

    for child in &mut paragraph.children {
        match child {
            ParagraphChild::CommentStart(c) => {
                refresh_para_ids_in_comment(&mut c.comment, counts, used, seen)
            }
            ParagraphChild::Insert(insert) => {
                refresh_para_ids_in_insert(insert, counts, used, seen)
            }
            ParagraphChild::Delete(delete) => {
                refresh_para_ids_in_delete(delete, counts, used, seen)
            }
            ParagraphChild::Hyperlink(hyperlink) => {
                refresh_para_ids_in_hyperlink(hyperlink, counts, used, seen)
            }
            ParagraphChild::StructuredDataTag(tag) => {
                refresh_para_ids_in_structured_data_tag(tag, counts, used, seen)
            }
            _ => {}
        }
    }
}

fn refresh_para_ids_in_hyperlink(
    hyperlink: &mut Hyperlink,
    counts: &HashMap<String, usize>,
    used: &mut HashSet<String>,
    seen: &mut HashSet<String>,
) {
    for child in &mut hyperlink.children {
        match child {
            ParagraphChild::CommentStart(c) => {
                refresh_para_ids_in_comment(&mut c.comment, counts, used, seen)
            }
            ParagraphChild::Insert(insert) => {
                refresh_para_ids_in_insert(insert, counts, used, seen)
            }
            ParagraphChild::Delete(delete) => {
                refresh_para_ids_in_delete(delete, counts, used, seen)
            }
            ParagraphChild::StructuredDataTag(tag) => {
                refresh_para_ids_in_structured_data_tag(tag, counts, used, seen)
            }
            _ => {}
        }
    }
}

fn refresh_para_ids_in_insert(
    insert: &mut Insert,
    counts: &HashMap<String, usize>,
    used: &mut HashSet<String>,
    seen: &mut HashSet<String>,
) {
    for child in &mut insert.children {
        match child {
            InsertChild::CommentStart(c) => {
                refresh_para_ids_in_comment(&mut c.comment, counts, used, seen)
            }
            InsertChild::Delete(delete) => refresh_para_ids_in_delete(delete, counts, used, seen),
            _ => {}
        }
    }
}

fn refresh_para_ids_in_delete(
    delete: &mut Delete,
    counts: &HashMap<String, usize>,
    used: &mut HashSet<String>,
    seen: &mut HashSet<String>,
) {
    for child in &mut delete.children {
        if let DeleteChild::CommentStart(c) = child {
            refresh_para_ids_in_comment(&mut c.comment, counts, used, seen);
        }
    }
}

fn refresh_para_ids_in_structured_data_tag(
    tag: &mut StructuredDataTag,
    counts: &HashMap<String, usize>,
    used: &mut HashSet<String>,
    seen: &mut HashSet<String>,
) {
    for child in &mut tag.children {
        match child {
            StructuredDataTagChild::Paragraph(paragraph) => {
                refresh_para_ids_in_paragraph(paragraph, counts, used, seen)
            }
            StructuredDataTagChild::Table(table) => {
                refresh_para_ids_in_table(table, counts, used, seen)
            }
            StructuredDataTagChild::CommentStart(c) => {
                refresh_para_ids_in_comment(&mut c.comment, counts, used, seen)
            }
            StructuredDataTagChild::StructuredDataTag(inner) => {
                refresh_para_ids_in_structured_data_tag(inner, counts, used, seen)
            }
            _ => {}
        }
    }
}

fn refresh_para_ids_in_comment(
    comment: &mut Comment,
    counts: &HashMap<String, usize>,
    used: &mut HashSet<String>,
    seen: &mut HashSet<String>,
) {
    for child in &mut comment.children {
        match child {
            CommentChild::Paragraph(paragraph) => {
                refresh_para_ids_in_paragraph(paragraph, counts, used, seen)
            }
            CommentChild::Table(table) => refresh_para_ids_in_table(table, counts, used, seen),
        }
    }
}

fn ensure_unique_paragraph_id(
    paragraph: &mut Paragraph,
    counts: &HashMap<String, usize>,
    used: &mut HashSet<String>,
    seen: &mut HashSet<String>,
) {
    let id = paragraph.id.clone();
    let count = counts.get(&id).copied().unwrap_or(0);

    if !id.is_empty() && count <= 1 {
        return;
    }
    if !id.is_empty() && count > 1 && !seen.contains(&id) {
        seen.insert(id);
        return;
    }

    let mut new_id = crate::generate_para_id();
    while used.contains(&new_id) {
        new_id = crate::generate_para_id();
    }
    paragraph.id = new_id.clone();
    used.insert(new_id);
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
