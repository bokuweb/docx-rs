mod delete;
mod document;
mod errors;
mod from_xml;
mod insert;
mod numbering_property;
mod paragraph;
mod run;
mod xml_element;

use delete::*;
use document::*;
use errors::*;
use insert::*;
use numbering_property::*;
use paragraph::*;
use run::*;
use serde_json;

use std::fs;
use std::io::Cursor;
use std::io::Read;
use xml::reader::{EventReader, XmlEvent};
use zip;

use crate::documents::*;

pub use errors::ReaderError;
pub use from_xml::*;
pub use xml_element::*;

const DOC_RELATIONSHIP_TYPE: &'static str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument";

pub fn read_docx(buf: &[u8]) -> Result<(), ReaderError> {
    let cur = Cursor::new(buf);
    let mut archive = zip::ZipArchive::new(cur)?;
    // First, the content type for relationship parts and the Main Document part
    // (the only required part) must be defined (physically located at /[Content_Types].xml in the package)
    let content_types_xml = archive.by_name("[Content_Types].xml")?;
    let content_types = ContentTypes::from_xml(content_types_xml)?;
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
    Document::from_xml(document_xml)?;
    Ok(())
}
