use super::*;
use serde::{Deserialize, Serialize};

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Graphic {
    data: GraphicData,
}

/*

*/
impl Graphic {
    pub fn new(data: GraphicData) -> Graphic {
        Self { data }
    }
}

impl BuildXML for Graphic {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        let mut b = b.open_graphic("http://schemas.openxmlformats.org/drawingml/2006/main");
        b = b.add_child(&self.data);
        b.close().build()
    }
}
