use serde::{Deserialize, Serialize};
use std::io::Write;

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DocVar {
    name: String,
    val: String,
}

impl DocVar {
    pub fn new(name: impl Into<String>, val: impl Into<String>) -> DocVar {
        DocVar {
            name: name.into(),
            val: val.into(),
        }
    }
}

impl BuildXML for DocVar {
    fn build_to<W: Write>(
        &self,
        stream: xml::writer::EventWriter<W>,
    ) -> xml::writer::Result<xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .doc_var(&self.name, &self.val)?
            .into_inner()
    }
}
