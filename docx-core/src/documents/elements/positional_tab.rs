use serde::{Deserialize, Serialize};

use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "wasm", derive(ts_rs::TS))]
#[cfg_attr(feature = "wasm", ts(export))]
pub struct PositionalTab {
    pub alignment: PositionalTabAlignmentType,
    pub relative_to: Option<PositionalTabRelativeTo>,
    pub leader: Option<TabLeaderType>,
}

impl PositionalTab {
    pub fn new(alignment: PositionalTabAlignmentType) -> Self {
        Self {
            alignment,
            relative_to: None,
            leader: None,
        }
    }

    pub fn relative_to(mut self, relative_to: PositionalTabRelativeTo) -> Self {
        self.relative_to = Some(relative_to);
        self
    }

    pub fn leader(mut self, v: TabLeaderType) -> Self {
        self.leader = Some(v);
        self
    }

    pub fn alignment(mut self, alignment: PositionalTabAlignmentType) -> Self {
        self.alignment = alignment;
        self
    }
}

impl BuildXML for PositionalTab {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.ptab(self.alignment, self.relative_to, self.leader)
            .build()
    }
}
