use serde::{Deserialize, Serialize, Serializer};

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct DocId {
    id: String,
}

impl DocId {
    pub fn new(id: impl Into<String>) -> DocId {
        DocId { id: id.into() }
    }
}

impl Serialize for DocId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.id)
    }
}

impl BuildXML for DocId {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        let id = format!("{{{}}}", self.id);
        b.doc_id(&id).build()
    }
}
