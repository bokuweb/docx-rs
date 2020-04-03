use serde::{Deserialize, Serialize};

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct GraphicData {}

impl GraphicData {
    pub fn new() -> GraphicData {
        Default::default()
    }
}

impl Default for GraphicData {
    fn default() -> Self {
        GraphicData {}
    }
}

impl BuildXML for GraphicData {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        let mut b = b.open_graphic_data("TODO:");
        b.close().build()
    }
}
