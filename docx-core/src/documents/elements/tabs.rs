use serde::{Deserialize, Serialize};
use std::io::Write;

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
    fn build_to<W: Write>(
        &self,
        stream: xml::writer::EventWriter<W>,
    ) -> xml::writer::Result<xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .apply_each(&self.tabs, |t, b| b.tab(t.val, t.leader, t.pos))?
            .into_inner()
    }
}
