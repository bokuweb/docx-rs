use super::*;
use image::*;
use serde::Serialize;

use crate::documents::*;
use crate::xml_builder::*;

#[derive(Debug, Clone, Copy, Serialize, PartialEq)]
pub enum PicAlign {
  Left,
  Right,
  Bottom,
  Top,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq)]
pub enum DrawingPosition {
  Offset(usize),
  Align(PicAlign),
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Pic {
  pub id: usize,
  pub image: Vec<u8>,
  pub size: (u32, u32),
  pub position_type: DrawingPositionType,
  pub position_h: DrawingPosition,
  pub position_v: DrawingPosition,
}

impl Pic {
  pub fn new(buf: Vec<u8>) -> Pic {
    let id = generate_pic_id();
    let dimg = image::load_from_memory(&buf).unwrap();
    let size = dimg.dimensions();
    let mut image = vec![];
    dimg
      .write_to(&mut image, ImageFormat::Png)
      .expect("Unable to write");
    Self {
      id,
      image,
      size,
      position_type: DrawingPositionType::Inline {
        dist_t: 0,
        dist_b: 0,
        dist_l: 0,
        dist_r: 0,
      },
      position_h: DrawingPosition::Offset(0),
      position_v: DrawingPosition::Offset(0),
    }
  }

  pub fn size(mut self, w_px: u32, h_px: u32) -> Pic {
    self.size = (w_px, h_px);
    self
  }

  pub fn floating(mut self) -> Pic {
    self.position_type = DrawingPositionType::Anchor;
    self
  }

  pub fn offset_x(mut self, x: usize) -> Pic {
    self.position_h = DrawingPosition::Offset(x);
    self
  }

  pub fn offset_y(mut self, y: usize) -> Pic {
    self.position_v = DrawingPosition::Offset(y);
    self
  }
}

impl BuildXML for Pic {
  fn build(&self) -> Vec<u8> {
    let b = XMLBuilder::new();
    let w = format!("{}", crate::types::emu::from_px(self.size.0));
    let h = format!("{}", crate::types::emu::from_px(self.size.1));
    b.open_pic("http://schemas.openxmlformats.org/drawingml/2006/picture")
      .open_pic_nv_pic_pr()
      .pic_c_nv_pr("0", "")
      .open_pic_c_nv_pic_pr()
      .a_pic_locks("1", "1")
      .close()
      .close()
      .open_blip_fill()
      .a_blip(&create_pic_rid(self.id))
      .a_src_rect()
      .open_a_stretch()
      .a_fill_rect()
      .close()
      .close()
      .open_pic_sp_pr("auto")
      .open_a_xfrm()
      .a_off("0", "0")
      .a_ext(&w, &h)
      .close()
      .open_a_prst_geom("rect")
      .a_av_lst()
      .close()
      .close()
      .close()
      .build()
  }
}

#[cfg(test)]
mod tests {

  use super::*;
  #[cfg(test)]
  use pretty_assertions::assert_eq;
  use std::str;

  #[test]
  fn test_pic_build() {
    use std::io::Read;

    let mut img = std::fs::File::open("../images/cat_min.jpg").unwrap();
    let mut buf = Vec::new();
    let _ = img.read_to_end(&mut buf).unwrap();
    let b = Pic::new(buf).build();
    assert_eq!(
      str::from_utf8(&b).unwrap(),
      r#"<pic:pic xmlns:pic="http://schemas.openxmlformats.org/drawingml/2006/picture">
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
    <a:xfrm>
      <a:off x="0" y="0" />
      <a:ext cx="3048000" cy="2286000" />
    </a:xfrm>
    <a:prstGeom prst="rect">
      <a:avLst />
    </a:prstGeom>
  </pic:spPr>
</pic:pic>"#
    );
  }
}
