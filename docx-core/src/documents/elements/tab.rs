use serde::{Deserialize, Serialize};

use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Default)]
pub struct Tab {
    pub val: Option<TabValueType>,
    pub leader: Option<TabLeaderType>,
    pub pos: Option<usize>,
}

impl Tab {
    pub fn new() -> Tab {
        Default::default()
    }
}

impl BuildXML for Tab {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.tab(self.val, self.leader, self.pos).build()
    }
}
