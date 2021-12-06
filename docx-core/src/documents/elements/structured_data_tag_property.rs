use serde::Serialize;

use super::*;
use crate::documents::BuildXML;
// use crate::types::*;
use crate::xml_builder::*;

#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StructuredDataTagProperty {
    pub run_property: RunProperty,
}

impl Default for StructuredDataTagProperty {
    fn default() -> Self {
        Self {
            run_property: RunProperty::new(),
        }
    }
}

impl StructuredDataTagProperty {
    pub fn new() -> Self {
        Default::default()
    }
}

impl BuildXML for StructuredDataTagProperty {
    fn build(&self) -> Vec<u8> {
        XMLBuilder::new()
            .open_structured_tag_property()
            .add_child(&self.run_property)
            .close()
            .build()
    }
}
