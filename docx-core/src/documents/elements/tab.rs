use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone)]
pub struct Tab {}

impl Tab {
    pub fn new() -> Tab {
        Tab {}
    }
}

impl BuildXML for Tab {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.tab().build()
    }
}
