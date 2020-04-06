use super::*;
use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Drawing {
    pub children: Vec<DrawingChild>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DrawingChild {
    WpAnchor(WpAnchor),
}

impl Serialize for DrawingChild {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            DrawingChild::WpAnchor(ref s) => {
                let mut t = serializer.serialize_struct("WpAnchor", 2)?;
                t.serialize_field("type", "anchor")?;
                t.serialize_field("data", s)?;
                t.end()
            }
        }
    }
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
