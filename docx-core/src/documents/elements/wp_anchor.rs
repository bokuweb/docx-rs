use super::*;
use serde::Serialize;
use std::io::Write;

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, Serialize, PartialEq, Default)]
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

impl BuildXML for WpAnchor {
    fn build_to<W: Write>(
        &self,
        stream: xml::writer::EventWriter<W>,
    ) -> xml::writer::Result<xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_anchor()?
            .add_children(&self.children)?
            .close()?
            .into_inner()
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
