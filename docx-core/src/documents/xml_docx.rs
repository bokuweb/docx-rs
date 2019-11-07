use super::XMLDocProps;
use wasm_bindgen::prelude::*;

#[derive(Debug)]
pub struct XMLDocx {
    pub content_type: Vec<u8>,
    pub rels: Vec<u8>,
    pub doc_props: XMLDocProps,
    pub styles: Vec<u8>,
    pub document: Vec<u8>,
}
