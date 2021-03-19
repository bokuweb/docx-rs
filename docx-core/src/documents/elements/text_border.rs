use serde::{Deserialize, Serialize};

use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextBorder {
    pub border_type: BorderType,
    pub size: usize,
    pub color: String,
    pub space: usize,
}

impl TextBorder {
    pub fn new() -> Self {
        TextBorder::default()
    }

    pub fn color(mut self, color: impl Into<String>) -> Self {
        self.color = color.into();
        self
    }

    pub fn size(mut self, size: usize) -> Self {
        self.size = size;
        self
    }

    pub fn space(mut self, space: usize) -> Self {
        self.space = space;
        self
    }

    pub fn border_type(mut self, border_type: BorderType) -> Self {
        self.border_type = border_type;
        self
    }
}

impl Default for TextBorder {
    fn default() -> Self {
        TextBorder {
            border_type: BorderType::Single,
            size: 4,
            space: 0,
            color: "auto".to_owned(),
        }
    }
}

impl BuildXML for TextBorder {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.text_border(self.border_type, self.size, self.space, &self.color)
            .build()
    }
}
