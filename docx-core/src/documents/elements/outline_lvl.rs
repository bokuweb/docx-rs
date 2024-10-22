use crate::documents::BuildXML;
use crate::xml_builder::*;

use serde::*;

#[derive(Debug, Clone, PartialEq)]
pub struct OutlineLvl {
    pub v: usize,
}

impl OutlineLvl {
    pub fn new(v: usize) -> OutlineLvl {
        assert!(v < 10, "outline level should be less than 10");
        OutlineLvl { v }
    }
}

impl BuildXML for OutlineLvl {
    fn build(&self) -> Vec<u8> {
        XMLBuilder::new(Vec::new())
            .outline_lvl(self.v)
            // .close()
            .into_inner()
    }
}

impl Serialize for OutlineLvl {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u32(self.v as u32)
    }
}

#[cfg(test)]
mod tests {
    use crate::{BuildXML, OutlineLvl};

    #[test]
    fn test_outline_lvl_build() {
        let bytes = OutlineLvl::new(1).build();
        assert_eq!(
            std::str::from_utf8(&bytes).unwrap(),
            r#"<w:outlineLvl w:val="1" />"#
        );
    }
}
