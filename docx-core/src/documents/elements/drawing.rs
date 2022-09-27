use super::*;
use serde::{ser::*, Serialize};

use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;

#[derive(Debug, Clone, PartialEq, Default, Serialize)]
pub struct Drawing {
    #[serde(flatten)]
    pub data: Option<DrawingData>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DrawingData {
    Pic(Pic),
    TextBox(TextBox),
}

impl Serialize for DrawingData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            DrawingData::Pic(ref pic) => {
                let mut t = serializer.serialize_struct("Pic", 2)?;
                t.serialize_field("type", "pic")?;
                t.serialize_field("data", pic)?;
                t.end()
            }
            DrawingData::TextBox(ref text_box) => {
                let mut t = serializer.serialize_struct("TextBox", 2)?;
                t.serialize_field("type", "textBox")?;
                t.serialize_field("data", text_box)?;
                t.end()
            }
        }
    }
}

impl Drawing {
    pub fn new() -> Drawing {
        Default::default()
    }

    pub fn pic(mut self, pic: Pic) -> Drawing {
        self.data = Some(DrawingData::Pic(pic));
        self
    }

    pub fn text_box(mut self, t: TextBox) -> Drawing {
        self.data = Some(DrawingData::TextBox(t));
        self
    }
}

impl BuildXML for Box<Drawing> {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        let mut b = b.open_drawing();

        match &self.data {
            Some(DrawingData::Pic(p)) => {
                if let DrawingPositionType::Inline { .. } = p.position_type {
                    b = b.open_wp_inline(
                        &format!("{}", p.dist_t),
                        &format!("{}", p.dist_b),
                        &format!("{}", p.dist_l),
                        &format!("{}", p.dist_r),
                    )
                } else {
                    b = b
                        .open_wp_anchor(
                            &format!("{}", p.dist_t),
                            &format!("{}", p.dist_b),
                            &format!("{}", p.dist_l),
                            &format!("{}", p.dist_r),
                            "0",
                            if p.simple_pos { "1" } else { "0" },
                            "0",
                            "0",
                            if p.layout_in_cell { "1" } else { "0" },
                            &format!("{}", p.relative_height),
                        )
                        .simple_pos(
                            &format!("{}", p.simple_pos_x),
                            &format!("{}", p.simple_pos_y),
                        )
                        .open_position_h(&format!("{}", p.relative_from_h));

                    if let DrawingPosition::Offset(x) = p.position_h {
                        let x = format!("{}", x as u32);
                        b = b.pos_offset(&x).close();
                    }

                    b = b.open_position_v(&format!("{}", p.relative_from_v));

                    if let DrawingPosition::Offset(y) = p.position_v {
                        let y = format!("{}", y as u32);
                        b = b.pos_offset(&y).close();
                    }
                }

                let w = format!("{}", p.size.0);
                let h = format!("{}", p.size.1);
                b = b
                    // Please see 20.4.2.7 extent (Drawing Object Size)
                    // One inch equates to 914400 EMUs and a centimeter is 360000
                    .wp_extent(&w, &h)
                    .wp_effect_extent("0", "0", "0", "0")
                    .wrap_none()
                    .wp_doc_pr("1", "Figure")
                    .open_wp_c_nv_graphic_frame_pr()
                    .a_graphic_frame_locks(
                        "http://schemas.openxmlformats.org/drawingml/2006/main",
                        "1",
                    )
                    .close()
                    .open_a_graphic("http://schemas.openxmlformats.org/drawingml/2006/main")
                    .open_a_graphic_data("http://schemas.openxmlformats.org/drawingml/2006/picture")
                    .add_child(&p.clone())
                    .close()
                    .close();
            }
            Some(DrawingData::TextBox(_t)) => unimplemented!("TODO: Support textBox writer"),
            None => {
                unimplemented!()
            }
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
        use std::io::Read;

        let mut img = std::fs::File::open("../images/cat_min.jpg").unwrap();
        let mut buf = Vec::new();
        let _ = img.read_to_end(&mut buf).unwrap();
        let d = Box::new(Drawing::new().pic(Pic::new(&buf))).build();
        assert_eq!(
            str::from_utf8(&d).unwrap(),
            r#"<w:drawing>
  <wp:inline distT="0" distB="0" distL="0" distR="0">
    <wp:extent cx="3048000" cy="2286000" />
    <wp:effectExtent b="0" l="0" r="0" t="0" />
    <wp:wrapNone />
    <wp:docPr id="1" name="Figure" />
    <wp:cNvGraphicFramePr>
      <a:graphicFrameLocks xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" noChangeAspect="1" />
    </wp:cNvGraphicFramePr>
    <a:graphic xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
      <a:graphicData uri="http://schemas.openxmlformats.org/drawingml/2006/picture"><pic:pic xmlns:pic="http://schemas.openxmlformats.org/drawingml/2006/picture">
  <pic:nvPicPr>
    <pic:cNvPr id="0" name="" />
    <pic:cNvPicPr>
      <a:picLocks noChangeAspect="1" noChangeArrowheads="1" />
    </pic:cNvPicPr>
  </pic:nvPicPr>
  <pic:blipFill>
    <a:blip r:embed="rIdImage123" />
    <a:srcRect />
    <a:stretch>
      <a:fillRect />
    </a:stretch>
  </pic:blipFill>
  <pic:spPr bwMode="auto">
    <a:xfrm rot="0">
      <a:off x="0" y="0" />
      <a:ext cx="3048000" cy="2286000" />
    </a:xfrm>
    <a:prstGeom prst="rect">
      <a:avLst />
    </a:prstGeom>
  </pic:spPr>
</pic:pic></a:graphicData>
    </a:graphic>
  </wp:inline>
</w:drawing>"#
        );
    }
}
