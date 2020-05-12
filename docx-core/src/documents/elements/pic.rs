use serde::Serialize;

use crate::documents::*;
use crate::xml_builder::*;

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Pic {
    pub id: usize,
    pub image: Vec<u8>,
}

impl Pic {
    pub fn new(buf: Vec<u8>) -> Pic {
        let id = generate_pic_id();
        Self { id, image: buf }
    }
}

impl BuildXML for Pic {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
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
            .a_ext("952500", "952500")
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
        let b = Pic::new(vec![0]).build();
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
      <a:ext cx="952500" cy="952500" />
    </a:xfrm>
    <a:prstGeom prst="rect">
      <a:avLst />
    </a:prstGeom>
  </pic:spPr>
</pic:pic>"#
        );
    }
}
