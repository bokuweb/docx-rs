use super::*;
use serde::{Deserialize, Serialize};

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Anchor {
    pub graphic: Option<Graphic>,
}

/*
  20.4.2.3
  anchor (Anchor for Floating DrawingML Object)
  This element specifies that the DrawingML object located at this position in the document is a floating object.
  Within a WordprocessingML document, drawing objects can exist in two states:
  - Inline - The drawing object is in line with the text, and affects the line height and layout of its line (like a
  - character glyph of similar size).
  Floating - The drawing object is anchored within the text, but can be absolutely positioned in the
  document relative to the page.
  When this element encapsulates the DrawingML object's i
*/
impl Anchor {
    pub fn new() -> Anchor {
        Default::default()
    }

    pub fn add_graphic(mut self, g: Graphic) -> Anchor {
        self.graphic = Some(g);
        self
    }
}

impl Default for Anchor {
    fn default() -> Self {
        Anchor { graphic: None }
    }
}

impl BuildXML for Anchor {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        let mut b = b.open_anchor();
        b = b.add_optional_child(&self.graphic);
        b.close().build()
    }
}
