use super::*;
use serde::Serialize;

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct WpTextBox {
    children: Vec<WpTextBoxChild>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum WpTextBoxChild {
    TextBoxContent(TextBoxContent),
}

impl WpTextBox {
    pub fn new() -> WpTextBox {
        Default::default()
    }
}

impl Default for WpTextBox {
    fn default() -> Self {
        WpTextBox { children: vec![] }
    }
}

impl BuildXML for WpTextBox {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        let mut b = b.open_wp_text_box();
        for c in &self.children {
            match c {
                WpTextBoxChild::TextBoxContent(t) => b = b.add_child(t),
            }
        }
        b.close().build()
    }
}
