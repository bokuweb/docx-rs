use serde::{Deserialize, Serialize};
use std::io::Write;

use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DocGrid {
    grid_type: DocGridType,
    line_pitch: Option<usize>,
    char_space: Option<isize>,
}

impl DocGrid {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_empty() -> Self {
        Self {
            grid_type: DocGridType::Default,
            line_pitch: None,
            char_space: None,
        }
    }

    pub fn grid_type(mut self, t: DocGridType) -> Self {
        self.grid_type = t;
        self
    }

    pub fn line_pitch(mut self, line_pitch: usize) -> Self {
        self.line_pitch = Some(line_pitch);
        self
    }

    pub fn char_space(mut self, char_space: isize) -> Self {
        self.char_space = Some(char_space);
        self
    }
}

impl Default for DocGrid {
    fn default() -> Self {
        Self {
            grid_type: DocGridType::Lines,
            line_pitch: Some(360),
            char_space: None,
        }
    }
}

impl BuildXML for DocGrid {
    fn build_to<W: Write>(
        &self,
        stream: xml::writer::EventWriter<W>,
    ) -> xml::writer::Result<xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .doc_grid(&self.grid_type, self.line_pitch, self.char_space)?
            .into_inner()
    }
}
