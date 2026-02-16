use super::*;
use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;
use std::io::Write;

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, Serialize, PartialEq, Default)]
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

impl BuildXML for WpsShapeChild {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        match self {
            WpsShapeChild::WpsTextBox(t) => t.build_to(stream),
        }
    }
}

impl BuildXML for WpsShape {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_wp_text_box()?
            .add_children(&self.children)?
            .close()?
            .into_inner()
    }
}
