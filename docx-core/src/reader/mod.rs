mod a_graphic;
mod a_graphic_data;
mod attributes;
mod bookmark_end;
mod bookmark_start;
mod delete;
mod document;
mod document_rels;
mod drawing;
mod errors;
mod from_xml;
mod ignore;
mod insert;
mod level;
mod level_override;
mod mc_fallback;
mod numbering_property;
mod numberings;
mod paragraph;
mod read_zip;
mod rels;
mod run;
mod run_property;
mod settings;
mod style;
mod styles;
mod table;
mod table_cell;
mod table_cell_borders;
mod table_row;
mod text_box_content;
mod wp_anchor;
mod wps_shape;
mod wps_text_box;
mod xml_element;

use std::io::Cursor;

use crate::documents::*;

pub use attributes::*;
pub use document_rels::*;
pub use errors::ReaderError;
pub use from_xml::*;
pub use mc_fallback::*;
pub use read_zip::*;
pub use xml_element::*;

const DOC_RELATIONSHIP_TYPE: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument";
const STYLE_RELATIONSHIP_TYPE: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles";
const NUMBERING_RELATIONSHIP_TYPE: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/numbering";
const SETTINGS_TYPE: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/settings";

pub fn read_docx(buf: &[u8]) -> Result<Docx, ReaderError> {
    let cur = Cursor::new(buf);
    let mut archive = zip::ZipArchive::new(cur)?;
    // First, the content type for relationship parts and the Main Document part
    // (the only required part) must be defined (physically located at /[Content_Types].xml in the package)
    let _content_types = {
        let data = read_zip(&mut archive, "[Content_Types].xml")?;
        ContentTypes::from_xml(&data[..])?
    };

    // Next, the single required relationship (the package-level relationship to the Main Document part)
    //  must be defined (physically located at /_rels/.rels in the package)
    let rels = {
        let data = read_zip(&mut archive, "_rels/.rels")?;
        Rels::from_xml(&data[..])?
    };
    // Finally, the minimum content for the Main Document part must be defined
    // (physically located at /document.xml in the package):
    let main_rel = rels
        .find_target(DOC_RELATIONSHIP_TYPE)
        .ok_or(ReaderError::DocumentNotFoundError)?;
    let document = {
        let data = read_zip(&mut archive, &main_rel.2)?;
        Document::from_xml(&data[..])?
    };
    let mut docx = Docx::new().document(document);
    // Read document relationships
    let rels = read_document_rels(&mut archive, &main_rel.2)?;

    // Read styles
    let style_path = rels.find_target_path(STYLE_RELATIONSHIP_TYPE);
    if let Some(style_path) = style_path {
        let data = read_zip(
            &mut archive,
            style_path.to_str().expect("should have styles"),
        )?;
        let styles = Styles::from_xml(&data[..])?;
        docx = docx.styles(styles);
    }

    // Read numberings
    let num_path = rels.find_target_path(NUMBERING_RELATIONSHIP_TYPE);
    if let Some(num_path) = num_path {
        let data = read_zip(
            &mut archive,
            num_path.to_str().expect("should have numberings"),
        )?;
        let nums = Numberings::from_xml(&data[..])?;
        docx = docx.numberings(nums);
    }

    // Read settings
    let settings_path = rels.find_target_path(SETTINGS_TYPE);
    if let Some(settings_path) = settings_path {
        let data = read_zip(
            &mut archive,
            settings_path.to_str().expect("should have settings"),
        )?;
        let settings = Settings::from_xml(&data[..])?;
        docx = docx.settings(settings);
    }

    Ok(docx)
}
