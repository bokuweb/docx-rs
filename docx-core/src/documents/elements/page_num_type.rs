use crate::documents::BuildXML;
use crate::xml_builder::*;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[cfg_attr(feature = "wasm", derive(ts_rs::TS))]
#[cfg_attr(feature = "wasm", ts(export))]
#[serde(rename_all = "camelCase")]
pub struct PageNumType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chap_style: Option<String>,
}

impl PageNumType {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn start(self, s: u32) -> Self {
        Self {
            start: Some(s),
            ..self
        }
    }

    pub fn chap_style(self, s: impl Into<String>) -> Self {
        Self {
            chap_style: Some(s.into()),
            ..self
        }
    }
}

impl BuildXML for PageNumType {
    fn build(&self) -> Vec<u8> {
        XMLBuilder::new(Vec::new())
            .page_num_type(self.start, self.chap_style.clone())
            .into_inner()
    }
}
