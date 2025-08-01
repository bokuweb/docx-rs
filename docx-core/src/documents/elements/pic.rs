use serde::Serialize;
use std::io::Write;

use crate::documents::*;
use crate::types::*;
use crate::xml_builder::*;

#[derive(Debug, Clone, Serialize, PartialEq)]
#[cfg_attr(feature = "wasm", derive(ts_rs::TS))]
#[cfg_attr(feature = "wasm", ts(export))]
#[serde(rename_all = "camelCase")]
pub struct Pic {
    pub id: String,
    // For writer only
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub image: Vec<u8>,
    // (width, height). unit is emu
    pub size: (u32, u32),
    pub position_type: DrawingPositionType,
    /// Specifies that this object shall be positioned using the positioning information in the
    /// simplePos child element (§20.4.2.13). This positioning, when specified, positions the
    /// object on the page by placing its top left point at the x-y coordinates specified by that
    /// element.
    pub simple_pos: bool,
    // unit is emu
    pub simple_pos_x: i32,
    pub simple_pos_y: i32,
    /// Specifies whether this DrawingML object (e.g., an image) is displayed behind the document text.
    pub behind_doc: bool,
    /// Specifies how this DrawingML object behaves when its anchor is located in a table cell;
    /// and its specified position would cause it to intersect with a table cell displayed in the
    /// document. That behavior shall be as follows:
    pub layout_in_cell: bool,
    /// Specifies the relative Z-ordering of all DrawingML objects in this document. Each floating
    /// DrawingML object shall have a Z-ordering value, which determines which object is
    /// displayed when any two objects intersect. Higher values shall indicate higher Z-order;
    /// lower values shall indicate lower Z-order.
    pub relative_height: u32,
    pub allow_overlap: bool,
    pub position_h: DrawingPosition,
    pub position_v: DrawingPosition,
    pub relative_from_h: RelativeFromHType,
    pub relative_from_v: RelativeFromVType,
    /// Specifies the minimum distance which shall be maintained between the top edge of this drawing object and any subsequent text within the document when this graphical object is displayed within the document's contents.,
    /// The distance shall be measured in EMUs (English Metric Units).,
    pub dist_t: i32,
    pub dist_b: i32,
    pub dist_l: i32,
    pub dist_r: i32,
    // deg
    pub rot: u16,
}

impl Pic {
    #[cfg(feature = "image")]
    /// Make a `Pic`.
    ///
    /// Converts the passed image to PNG internally and computes its size.
    pub fn new(buf: &[u8]) -> Pic {
        let img = ::image::load_from_memory(buf).expect("Should load image from memory.");
        let (w, h) = ::image::GenericImageView::dimensions(&img);
        let mut buf = std::io::Cursor::new(vec![]);
        img.write_to(&mut buf, ::image::ImageFormat::Png)
            .expect("Unable to write dynamic image");
        Self::new_with_dimensions(buf.into_inner(), w, h)
    }

    /// Make a `Pic` element. For now only PNG is supported.
    ///
    /// Use [Pic::new] method, to call `image` crate do conversion for you.
    pub fn new_with_dimensions(buffer: Vec<u8>, width_px: u32, height_px: u32) -> Pic {
        let id = create_pic_rid(generate_pic_id());
        Self {
            id,
            image: buffer,
            size: (from_px(width_px), from_px(height_px)),
            position_type: DrawingPositionType::Inline,
            simple_pos: false,
            simple_pos_x: 0,
            simple_pos_y: 0,
            behind_doc: false,
            layout_in_cell: false,
            relative_height: 190500,
            allow_overlap: false,
            position_v: DrawingPosition::Offset(0),
            position_h: DrawingPosition::Offset(0),
            relative_from_h: RelativeFromHType::default(),
            relative_from_v: RelativeFromVType::default(),
            dist_t: 0,
            dist_b: 0,
            dist_l: 0,
            dist_r: 0,
            rot: 0,
        }
    }

    pub(crate) fn with_empty() -> Pic {
        Self {
            id: "".to_string(),
            image: vec![],
            size: (0, 0),
            position_type: DrawingPositionType::Inline,
            simple_pos: false,
            simple_pos_x: 0,
            simple_pos_y: 0,
            behind_doc: false,
            layout_in_cell: false,
            relative_height: 190500,
            allow_overlap: false,
            position_v: DrawingPosition::Offset(0),
            position_h: DrawingPosition::Offset(0),
            relative_from_h: RelativeFromHType::default(),
            relative_from_v: RelativeFromVType::default(),
            dist_t: 0,
            dist_b: 0,
            dist_l: 0,
            dist_r: 0,
            rot: 0,
        }
    }

    pub fn id(mut self, id: impl Into<String>) -> Pic {
        self.id = id.into();
        self
    }

    // unit is emu
    pub fn size(mut self, w_emu: u32, h_emu: u32) -> Pic {
        self.size = (w_emu, h_emu);
        self
    }

    // unit is deg
    pub fn rotate(mut self, deg: u16) -> Pic {
        self.rot = deg;
        self
    }

    pub fn floating(mut self) -> Pic {
        self.position_type = DrawingPositionType::Anchor;
        self
    }

    pub fn behind(mut self) -> Pic {
        self.behind_doc = true;
        self
    }

    pub fn overlapping(mut self) -> Pic {
        self.allow_overlap = true;
        self
    }

    pub fn offset_x(mut self, x: i32) -> Pic {
        self.position_h = DrawingPosition::Offset(x);
        self
    }

    pub fn offset_y(mut self, y: i32) -> Pic {
        self.position_v = DrawingPosition::Offset(y);
        self
    }

    pub fn position_h(mut self, pos: DrawingPosition) -> Self {
        self.position_h = pos;
        self
    }

    pub fn position_v(mut self, pos: DrawingPosition) -> Self {
        self.position_v = pos;
        self
    }

    pub fn relative_from_h(mut self, t: RelativeFromHType) -> Self {
        self.relative_from_h = t;
        self
    }

    pub fn relative_from_v(mut self, t: RelativeFromVType) -> Self {
        self.relative_from_v = t;
        self
    }

    pub fn dist_t(mut self, v: i32) -> Self {
        self.dist_t = v;
        self
    }

    pub fn dist_b(mut self, v: i32) -> Self {
        self.dist_b = v;
        self
    }

    pub fn dist_l(mut self, v: i32) -> Self {
        self.dist_l = v;
        self
    }

    pub fn dist_r(mut self, v: i32) -> Self {
        self.dist_r = v;
        self
    }

    pub fn simple_pos(mut self, v: bool) -> Self {
        self.simple_pos = v;
        self
    }

    pub fn relative_height(mut self, v: u32) -> Self {
        self.relative_height = v;
        self
    }
}

impl BuildXML for Pic {
    fn build_to<W: Write>(
        &self,
        stream: xml::writer::EventWriter<W>,
    ) -> xml::writer::Result<xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_pic("http://schemas.openxmlformats.org/drawingml/2006/picture")?
            .open_pic_nv_pic_pr()?
            .pic_c_nv_pr("0", "")?
            .open_pic_c_nv_pic_pr()?
            .a_pic_locks("1", "1")?
            .close()?
            .close()?
            .open_blip_fill()?
            .a_blip(&self.id)?
            .a_src_rect()?
            .open_a_stretch()?
            .a_fill_rect()?
            .close()?
            .close()?
            .open_pic_sp_pr("auto")?
            .open_a_xfrm_with_rot(&format!("{}", (self.rot as u32) * 60 * 1000))?
            .a_off("0", "0")?
            .a_ext(&format!("{}", self.size.0), &format!("{}", self.size.1))?
            .close()?
            .open_a_prst_geom("rect")?
            .a_av_lst()?
            .close()?
            .close()?
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
    fn test_pic_build() {
        let b = Pic::new_with_dimensions(Vec::new(), 320, 240).build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<pic:pic xmlns:pic="http://schemas.openxmlformats.org/drawingml/2006/picture"><pic:nvPicPr><pic:cNvPr id="0" name="" /><pic:cNvPicPr><a:picLocks noChangeAspect="1" noChangeArrowheads="1" /></pic:cNvPicPr></pic:nvPicPr><pic:blipFill><a:blip r:embed="rIdImage123" /><a:srcRect /><a:stretch><a:fillRect /></a:stretch></pic:blipFill><pic:spPr bwMode="auto"><a:xfrm rot="0"><a:off x="0" y="0" /><a:ext cx="3048000" cy="2286000" /></a:xfrm><a:prstGeom prst="rect"><a:avLst /></a:prstGeom></pic:spPr></pic:pic>"#
        );
    }
}
