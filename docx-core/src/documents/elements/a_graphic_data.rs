use super::*;
use serde::Serialize;
use std::str::FromStr;

use crate::documents::BuildXML;
use crate::xml_builder::*;

/*
  20.1.2.2.17
  graphicData (Graphic Object Data)
  This element specifies the reference to a graphic object within the document. This graphic object is provided
  entirely by the document authors who choose to persist this data within the document.
*/
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct AGraphicData {
    pub data_type: GraphicDataType,
    pub children: Vec<GraphicDataChild>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum GraphicDataChild {
    WpsShape(WpsShape),
}

impl GraphicDataType {
    fn into_uri(&self) -> &str {
        match *self {
            GraphicDataType::Picture => "http://schemas.openxmlformats.org/drawingml/2006/picture",
            GraphicDataType::WpShape => {
                "http://schemas.microsoft.com/office/word/2010/wordprocessingShape"
            }
            _ => "",
        }
    }
}

impl FromStr for GraphicDataType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.ends_with("picture") {
            return Ok(GraphicDataType::Picture);
        }
        if s.ends_with("wordprocessingShape") {
            return Ok(GraphicDataType::WpShape);
        }
        return Ok(GraphicDataType::Unsupported);
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum GraphicDataType {
    Picture,
    WpShape,
    Unsupported,
}

impl AGraphicData {
    pub fn new(data_type: GraphicDataType) -> AGraphicData {
        AGraphicData {
            data_type,
            children: vec![],
        }
    }
}

impl BuildXML for AGraphicData {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        let mut b = b.open_graphic_data(self.data_type.into_uri());
        for c in &self.children {
            match c {
                GraphicDataChild::WpsShape(t) => b = b.add_child(t),
            }
        }
        b.close().build()
    }
}
