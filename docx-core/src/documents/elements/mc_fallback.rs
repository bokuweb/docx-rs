use crate::documents::BuildXML;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq, Default)]
pub struct McFallback {}

impl McFallback {
    pub fn new() -> McFallback {
        Default::default()
    }
}

impl BuildXML for McFallback {
    fn build(&self) -> Vec<u8> {
        //  Ignore for now
        vec![]
    }
}
