use serde::{Deserialize, Serialize, Serializer};
use std::io::Write;

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
    fn build_to<W: Write>(
        &self,
        stream: xml::writer::EventWriter<W>,
    ) -> xml::writer::Result<xml::writer::EventWriter<W>> {
        let id = format!("{{{}}}", self.id);
        XMLBuilder::from(stream).doc_id(&id)?.into_inner()
    }
}
