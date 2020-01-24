use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone)]
pub struct BoldCs {}

impl BoldCs {
    pub fn new() -> BoldCs {
        Default::default()
    }
}

impl Default for BoldCs {
    fn default() -> Self {
        Self {}
    }
}

impl BuildXML for BoldCs {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.b_cs().build()
    }
}
