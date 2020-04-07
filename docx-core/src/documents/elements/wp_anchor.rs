use super::*;
use serde::Serialize;

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WpAnchor {
    pub children: Vec<AGraphic>,
}

/*
  20.4.2.3
  anchor (WpAnchor for Floating DrawingML Object)
  This element specifies that the DrawingML object located at this position in the document is a floating object.
  Within a WordprocessingML document, drawing objects can exist in two states:
  - Inline - The drawing object is in line with the text, and affects the line height and layout of its line (like a
  - character glyph of similar size).
  Floating - The drawing object is anchored within the text, but can be absolutely positioned in the
  document relative to the page.
  When this element encapsulates the DrawingML object's i
*/
impl WpAnchor {
    pub fn new() -> WpAnchor {
        Default::default()
    }

    pub fn add_graphic(mut self, g: AGraphic) -> WpAnchor {
        self.children.push(g);
        self
    }
}

impl Default for WpAnchor {
    fn default() -> Self {
        WpAnchor { children: vec![] }
    }
}

impl BuildXML for WpAnchor {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        let mut b = b.open_anchor();
        for c in &self.children {
            b = b.add_child(c)
        }
        b.close().build()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_anchor_build() {
        let b = WpAnchor::new().build();
        assert_eq!(str::from_utf8(&b).unwrap(), r#"<wp:anchor />"#);
    }
}
