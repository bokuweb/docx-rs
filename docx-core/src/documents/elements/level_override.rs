use crate::documents::BuildXML;
// use crate::xml_builder::*;

use serde::Serialize;

/*
17.9.8 lvlOverride (Numbering Level Definition Override)
This element specifies an optional override which shall be applied in place of zero or more levels from the abstract numbering definition for a given numbering definition instance. Each instance of this element is used to override the appearance and behavior of a given numbering level definition within the given abstract numbering definition.
*/
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LevelOverride {
    pub level: usize,
    pub start: Option<usize>,
}

impl LevelOverride {
    pub fn new(level: usize) -> LevelOverride {
        LevelOverride { level, start: None }
    }

    pub fn start(mut self, start: usize) -> LevelOverride {
        self.start = Some(start);
        self
    }
}

// TODO: Now read only
impl BuildXML for LevelOverride {
    fn build(&self) -> Vec<u8> {
        vec![]
    }
}
