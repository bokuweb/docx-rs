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
    pub document_rels: Vec<u8>,
    pub settings: Vec<u8>,
    pub font_table: Vec<u8>,
}

impl XMLDocx {
    pub fn pack<W>(self, w: W) -> zip::result::ZipResult<()>
    where
        W: Write + Seek,
    {
        zipper::zip(w, self)
    }
}
