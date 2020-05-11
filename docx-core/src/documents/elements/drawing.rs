use super::*;
use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Drawing {
    pub position_type: DrawingPositionType,
    pub data: Option<DrawingData>,
    // TODO: Old definition, remove later
    pub children: Vec<DrawingChild>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum DrawingData {
    Pic(Pic),
}

// TODO: Old definition, remove later
#[derive(Debug, Clone, PartialEq)]
pub enum DrawingChild {
    WpAnchor(WpAnchor),
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum DrawingPositionType {
    Anchor,
    Inline {
        dist_t: usize,
        dist_b: usize,
        dist_l: usize,
        dist_r: usize,
    },
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

    pub fn pic(mut self, pic: Pic) -> Drawing {
        self.data = Some(DrawingData::Pic(pic));
        self
    }
}

impl Default for Drawing {
    fn default() -> Self {
        Drawing {
            position_type: DrawingPositionType::Inline {
                dist_t: 0,
                dist_b: 0,
                dist_l: 0,
                dist_r: 0,
            },
            data: None,
            children: vec![],
        }
    }
}

impl BuildXML for Drawing {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        let mut b = b.open_drawing();

        if let DrawingPositionType::Inline { .. } = self.position_type {
            b = b
                .open_wp_inline("0", "0", "0", "0")
                .wp_extent("952500", "952599")
                .wp_effect_extent("0", "0", "0", "0")
                .wp_doc_pr("0", "", "")
                .open_wp_c_nv_graphic_frame_pr()
                .a_graphic_frame_locks("http://schemas.openxmlformats.org/drawingml/2006/main", "1")
                .close()
        } else {
            b = b.open_wp_anchor("0", "0", "0", "0");
        }
        match &self.data {
            Some(DrawingData::Pic(p)) => {
                b = b.add_child(&p.clone());
            }
            None => {}
        }
        b.close().close().build()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_drawing_build_with_pic() {
        let d = Drawing::new().pic(Pic::new(vec![0])).build();
        
        assert_eq!(
            str::from_utf8(&d).unwrap(),
            r#"<w:drawing>
  <wp:inline distT="0" distB="0" distL="0" distR="0">
    <wp:extent cx="952500" cy="952599" />
    <wp:effectExtent b="0" l="0" r="0" t="0" />
    <wp:doc_pr id="0" name="" descr="" />
    <a:cNvGraphicFramePr>
      <a:graphicFrameLocks xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" noChangeAspect="1" />
    </a:cNvGraphicFramePr><pic:pic xmlns:pic="http://schemas.openxmlformats.org/drawingml/2006/picture">
  <pic:nvPicPr />
  <pic:blipFill>
    <a:blip r:embed="rIdImage123" cstate="none" />
    <a:srcRect />
    <a:stretch>
      <a:fillRect />
    </a:stretch>
  </pic:blipFill>
  <pic:spPr bwMode="auto" />
</pic:pic></wp:inline>
</w:drawing>"#
        );
    }
}
