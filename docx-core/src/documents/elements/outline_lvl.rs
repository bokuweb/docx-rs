use crate::documents::BuildXML;
use crate::xml_builder::*;

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OutlineLvl {
    pub v: usize,
}

impl OutlineLvl {
    pub fn new(v: usize) -> OutlineLvl {
        OutlineLvl { v }
    }
}

impl BuildXML for OutlineLvl {
    fn build(&self) -> Vec<u8> {
        XMLBuilder::new()
            .outline_lvl(self.v)
            // .close()
            .build()
    }
}

#[cfg(test)]
mod tests {
    use crate::{BuildXML, OutlineLvl};


    #[test]
    fn test_outline_lvl_build(){
        let bytes = OutlineLvl::new(1).build();
        assert_eq!(std::str::from_utf8(&bytes).unwrap(),r#"<w:outlineLvl w:val="1" />"#);
    }

}
