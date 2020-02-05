use serde::{Deserialize, Serialize};

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ItalicCs {}

impl ItalicCs {
    pub fn new() -> ItalicCs {
        Default::default()
    }
}

impl Default for ItalicCs {
    fn default() -> Self {
        Self {}
    }
}

impl BuildXML for ItalicCs {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.i_cs().build()
    }
}
