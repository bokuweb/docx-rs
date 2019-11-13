use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone)]
pub struct Italic {}

impl Italic {
    pub fn new() -> Italic {
        Italic {}
    }
}

impl BuildXML for Italic {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.i().build()
    }
}
