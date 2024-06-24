use super::XMLDocProps;

use crate::zipper;
use std::io::prelude::*;
use std::io::Seek;

#[derive(Debug)]
pub struct XMLDocx {
    pub content_type: Vec<u8>,
    pub rels: Vec<u8>,
    pub doc_props: XMLDocProps,
    pub styles: Vec<u8>,
    pub document: Vec<u8>,
    pub comments: Vec<u8>,
    pub document_rels: Vec<u8>,
    pub settings: Vec<u8>,
    pub font_table: Vec<u8>,
    pub numberings: Vec<u8>,
    pub media: Vec<(String, Vec<u8>)>,
    pub headers: Vec<Vec<u8>>,
    pub header_rels: Vec<Vec<u8>>,
    pub footers: Vec<Vec<u8>>,
    pub footer_rels: Vec<Vec<u8>>,
    pub comments_extended: Vec<u8>,
    pub taskpanes: Option<Vec<u8>>,
    pub taskpanes_rels: Vec<u8>,
    pub web_extensions: Vec<Vec<u8>>,
    pub custom_items: Vec<Vec<u8>>,
    pub custom_item_rels: Vec<Vec<u8>>,
    pub custom_item_props: Vec<Vec<u8>>,
    pub footnotes: Vec<u8>,
}

impl XMLDocx {
    pub fn pack<W>(self, w: W) -> zip::result::ZipResult<()>
    where
        W: Write + Seek,
    {
        zipper::zip(w, self)
    }
}
