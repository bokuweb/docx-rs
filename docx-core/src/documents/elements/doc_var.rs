use serde::{Deserialize, Serialize};

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
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.doc_var(&self.name, &self.val).build()
    }
}
