use serde::{Deserialize, Serialize};

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
pub struct DocId {
    id: String,
}

impl DocId {
    pub fn new(id: impl Into<String>) -> DocId {
        DocId { id: id.into() }
    }
}

impl BuildXML for DocId {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        let id = format!("{{{}}}", self.id);
        b.doc_id(&id).build()
    }
}
