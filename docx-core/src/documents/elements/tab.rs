use serde::{Deserialize, Serialize};
use std::io::Write;

use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Default)]
#[cfg_attr(feature = "wasm", derive(ts_rs::TS))]
#[cfg_attr(feature = "wasm", ts(export))]
pub struct Tab {
    pub val: Option<TabValueType>,
    pub leader: Option<TabLeaderType>,
    pub pos: Option<usize>,
}

impl Tab {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn val(mut self, v: TabValueType) -> Self {
        self.val = Some(v);
        self
    }

    pub fn leader(mut self, v: TabLeaderType) -> Self {
        self.leader = Some(v);
        self
    }

    pub fn pos(mut self, v: usize) -> Self {
        self.pos = Some(v);
        self
    }
}

impl BuildXML for Tab {
    fn build_to<W: Write>(
        &self,
        stream: xml::writer::EventWriter<W>,
    ) -> xml::writer::Result<xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .tab(self.val, self.leader, self.pos)?
            .into_inner()
    }
}
