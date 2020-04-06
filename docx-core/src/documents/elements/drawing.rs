use super::*;
use serde::Serialize;

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Drawing {
    pub children: Vec<DrawingChild>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum DrawingChild {
    WpAnchor(WpAnchor),
}

impl Drawing {
    pub fn new() -> Drawing {
        Default::default()
    }

    pub fn add_anchor(mut self, a: WpAnchor) -> Drawing {
        self.children.push(DrawingChild::WpAnchor(a));
        self
    }
}

impl Default for Drawing {
    fn default() -> Self {
        Drawing { children: vec![] }
    }
}

impl BuildXML for Drawing {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        let mut b = b.open_drawing();
        for child in &self.children {
            match child {
                DrawingChild::WpAnchor(a) => {
                    b = b.add_child(a);
                }
            }
        }
        b.close().build()
    }
}
