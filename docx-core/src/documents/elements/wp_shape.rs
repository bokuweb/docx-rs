use super::*;
use serde::Serialize;

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct WpShape {
    children: Vec<WpShapeChild>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum WpShapeChild {
    WpTextBox(WpTextBox),
}

impl WpShape {
    pub fn new() -> WpShape {
        Default::default()
    }
}

impl Default for WpShape {
    fn default() -> Self {
        WpShape { children: vec![] }
    }
}

impl BuildXML for WpShape {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        let mut b = b.open_wp_text_box();
        for c in &self.children {
            match c {
                WpShapeChild::WpTextBox(t) => b = b.add_child(t),
            }
        }
        b.close().build()
    }
}
