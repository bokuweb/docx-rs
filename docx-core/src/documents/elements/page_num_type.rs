use crate::documents::BuildXML;
use crate::xml_builder::*;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PageNumType {
    pub start: Option<u32>,
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
        XMLBuilder::new()
            .page_num_type(self.start, self.chap_style.clone())
            .build()
    }
}
