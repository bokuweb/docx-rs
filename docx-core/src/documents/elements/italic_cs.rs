use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone)]
pub struct ItalicCs {}

impl ItalicCs {
    pub fn new() -> ItalicCs {
        ItalicCs {}
    }
}

impl BuildXML for ItalicCs {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.i_cs().build()
    }
}
