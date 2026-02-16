use serde::{Deserialize, Serialize};
use std::io::Write;

use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[cfg_attr(feature = "wasm", derive(ts_rs::TS))]
#[cfg_attr(feature = "wasm", ts(export))]
#[serde(rename_all = "camelCase")]
pub struct PositionalTab {
    pub alignment: PositionalTabAlignmentType,
    pub relative_to: PositionalTabRelativeTo,
    pub leader: TabLeaderType,
}

impl Default for PositionalTab {
    fn default() -> Self {
        Self {
            alignment: PositionalTabAlignmentType::Left,
            relative_to: PositionalTabRelativeTo::Margin,
            leader: TabLeaderType::None,
        }
    }
}

impl PositionalTab {
    pub fn new(
        alignment: PositionalTabAlignmentType,
        relative_to: PositionalTabRelativeTo,
        leader: TabLeaderType,
    ) -> Self {
        Self {
            alignment,
            relative_to,
            leader,
        }
    }

    pub fn relative_to(mut self, relative_to: PositionalTabRelativeTo) -> Self {
        self.relative_to = relative_to;
        self
    }

    pub fn leader(mut self, leader: TabLeaderType) -> Self {
        self.leader = leader;
        self
    }

    pub fn alignment(mut self, alignment: PositionalTabAlignmentType) -> Self {
        self.alignment = alignment;
        self
    }
}

impl BuildXML for PositionalTab {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .ptab(self.alignment, self.relative_to, self.leader)?
            .into_inner()
    }
}
