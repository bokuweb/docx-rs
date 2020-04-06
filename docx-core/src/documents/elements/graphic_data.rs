use serde::{Deserialize, Serialize};

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct GraphicData {
    r#type: GraphicDataType,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum GraphicDataType {
    Picture,
    Shape,
}

impl GraphicData {
    pub fn new(r#type: GraphicDataType) -> GraphicData {
        GraphicData { r#type }
    }
}

impl BuildXML for GraphicData {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        let mut b = b.open_graphic_data("TODO:");
        b.close().build()
    }
}
