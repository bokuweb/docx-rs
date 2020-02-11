use serde::{Deserialize, Serialize};

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Tab {}

impl Tab {
    pub fn new() -> Tab {
        Default::default()
    }
}

impl Default for Tab {
    fn default() -> Self {
        Tab {}
    }
}

impl BuildXML for Tab {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.tab().build()
    }
}
