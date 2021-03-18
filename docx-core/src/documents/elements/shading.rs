use serde::Serialize;

use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;

// INFO: Theme is not supported now.
#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Shading {
    pub shd_type: ShdType,
    pub color: String,
    pub fill: String,
}

impl Default for Shading {
    fn default() -> Self {
        Shading {
            shd_type: ShdType::Clear,
            color: "auto".to_owned(),
            fill: "FFFFFF".to_owned(),
        }
    }
}

impl Shading {
    pub fn new() -> Shading {
        Shading::default()
    }

    pub fn color(mut self, color: impl Into<String>) -> Shading {
        self.color = color.into();
        self
    }

    pub fn fill(mut self, fill: impl Into<String>) -> Shading {
        self.fill = fill.into();
        self
    }

    pub fn shd_type(mut self, shd_type: ShdType) -> Shading {
        self.shd_type = shd_type;
        self
    }
}

impl BuildXML for Shading {
    fn build(&self) -> Vec<u8> {
        XMLBuilder::new()
            .shd(&self.shd_type.to_string(), &self.color, &self.fill)
            .build()
    }
}
