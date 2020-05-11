use super::*;
use serde::Serialize;

use crate::documents::*;
use crate::xml_builder::*;

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Pic {
    id: String,
    image: Vec<u8>,
}

impl Pic {
    pub fn new(buf: Vec<u8>) -> Pic {
        let id = generate_pic_id();
        Self {
            id: format!("rIdImage{}", id),
            image: buf,
        }
    }
}

impl BuildXML for Pic {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.open_pic("http://schemas.openxmlformats.org/drawingml/2006/picture")
            .open_pic_nv_piv_pr()
            .close()
            .open_blip_fill()
            .a_blip(&self.id, "none")
            .a_src_rect()
            .open_a_stretch()
            .a_fill_rect()
            .close()
            .close()
            .open_pic_sp_pr("auto")
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
  <pic:nvPicPr />
  <pic:blipFill>
    <a:blip r:embed="rIdImage123" cstate="none" />
    <a:srcRect />
    <a:stretch>
      <a:fillRect />
    </a:stretch>
  </pic:blipFill>
  <pic:spPr bwMode="auto" />
</pic:pic>"#
        );
    }
}
