mod attributes;
mod delete;
mod document;
mod document_rels;
mod errors;
mod from_xml;
mod insert;
mod level;
mod numbering_property;
mod numberings;
mod paragraph;
mod rels;
mod run;
mod style;
mod styles;
mod table;
mod table_cell;
mod table_row;
mod xml_element;

use std::io::Cursor;
use zip;

use crate::documents::*;

pub use attributes::*;
pub use document_rels::*;
pub use errors::ReaderError;
pub use from_xml::*;
pub use xml_element::*;

const DOC_RELATIONSHIP_TYPE: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument";
const STYLE_RELATIONSHIP_TYPE: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles";
const NUMBERING_RELATIONSHIP_TYPE: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/numbering";

pub fn read_docx(buf: &[u8]) -> Result<Docx, ReaderError> {
    let cur = Cursor::new(buf);
    let mut archive = zip::ZipArchive::new(cur)?;
    // First, the content type for relationship parts and the Main Document part
    // (the only required part) must be defined (physically located at /[Content_Types].xml in the package)
    let content_types_xml = archive.by_name("[Content_Types].xml")?;
    let _content_types = ContentTypes::from_xml(content_types_xml)?;
    // Next, the single required relationship (the package-level relationship to the Main Document part)
    //  must be defined (physically located at /_rels/.rels in the package)
    let rels_xml = archive.by_name("_rels/.rels")?;
    let rels = Rels::from_xml(rels_xml)?;
    // Finally, the minimum content for the Main Document part must be defined
    // (physically located at /document.xml in the package):
    let main_rel = rels
        .find_target(DOC_RELATIONSHIP_TYPE)
        .ok_or(ReaderError::DocumentNotFoundError)?;
    let document_xml = archive.by_name(&main_rel.2)?;
    let document = Document::from_xml(document_xml)?;

    // Read document relationships
    let rels = read_document_rels(&mut archive, &main_rel.2)?;

    // Read styles
    let style_path = rels
        .find_target_path(STYLE_RELATIONSHIP_TYPE)
        .ok_or(ReaderError::DocumentStylesNotFoundError)?;
    let styles_xml = archive.by_name(style_path.to_str().expect("should have styles"))?;
    let styles = Styles::from_xml(styles_xml)?;

    // Read numberings
    let num_path = rels
        .find_target_path(NUMBERING_RELATIONSHIP_TYPE)
        .ok_or(ReaderError::DocumentStylesNotFoundError)?;
    let num_xml = archive.by_name(num_path.to_str().expect("should have numberings"))?;
    let nums = Numberings::from_xml(num_xml)?;

    let docx = Docx::new()
        .document(document)
        .styles(styles)
        .numberings(nums);
    Ok(docx)
}
