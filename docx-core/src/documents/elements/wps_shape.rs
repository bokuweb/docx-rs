use super::*;
use serde::Serialize;

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct WpsShape {
    children: Vec<WpsShapeChild>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum WpsShapeChild {
    WpsTextBox(WpsTextBox),
}

impl WpsShape {
    pub fn new() -> WpsShape {
        Default::default()
    }

    pub fn add_text_box(mut self, text_box: WpsTextBox) -> Self {
        self.children.push(WpsShapeChild::WpsTextBox(text_box));
        self
    }
}

impl Default for WpsShape {
    fn default() -> Self {
        WpsShape { children: vec![] }
    }
}

impl BuildXML for WpsShape {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        let mut b = b.open_wp_text_box();
        for c in &self.children {
            match c {
                WpsShapeChild::WpsTextBox(t) => b = b.add_child(t),
            }
        }
        b.close().build()
    }
}
