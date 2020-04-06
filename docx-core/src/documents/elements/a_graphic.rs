use super::*;
use serde::Serialize;

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct AGraphic {
    pub children: Vec<AGraphicData>,
}

impl AGraphic {
    pub fn new() -> AGraphic {
        Default::default()
    }

    pub fn add_graphic(mut self, g: AGraphicData) -> Self {
        self.children.push(g);
        self
    }
}

impl Default for AGraphic {
    fn default() -> Self {
        Self { children: vec![] }
    }
}

impl BuildXML for AGraphic {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        let mut b = b.open_graphic("http://schemas.openxmlformats.org/drawingml/2006/main");
        for child in &self.children {
            b = b.add_child(child);
        }
        b.close().build()
    }
}
