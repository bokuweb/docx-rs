use serde::{Deserialize, Serialize};

use crate::documents::BuildXML;
use crate::xml_builder::*;
use crate::Tab;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Default)]
pub struct Tabs {
    pub tabs: Vec<Tab>,
}

impl Tabs {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_tab(mut self, t: Tab) -> Self {
        self.tabs.push(t);
        self
    }
}

impl BuildXML for Tabs {
    fn build(&self) -> Vec<u8> {
        let mut b = XMLBuilder::new(Vec::new());
        for t in self.tabs.iter() {
            b = b.tab(t.val, t.leader, t.pos);
        }
        b.into_inner()
    }
}
