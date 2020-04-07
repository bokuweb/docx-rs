use super::*;
use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WpsShape {
    children: Vec<WpsShapeChild>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum WpsShapeChild {
    WpsTextBox(WpsTextBox),
}

impl Serialize for WpsShapeChild {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            WpsShapeChild::WpsTextBox(ref s) => {
                let mut t = serializer.serialize_struct("WpsTextBox", 2)?;
                t.serialize_field("type", "textbox")?;
                t.serialize_field("data", s)?;
                t.end()
            }
        }
    }
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
