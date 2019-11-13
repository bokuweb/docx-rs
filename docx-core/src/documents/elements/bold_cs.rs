use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone)]
pub struct BoldCs {}

impl BoldCs {
    pub fn new() -> BoldCs {
        BoldCs {}
    }
}

impl BuildXML for BoldCs {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.b_cs().build()
    }
}
