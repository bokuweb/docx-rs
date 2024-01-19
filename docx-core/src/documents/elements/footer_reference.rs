use crate::documents::BuildXML;
use crate::xml_builder::*;

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "wasm", derive(ts_rs::TS))]
#[cfg_attr(feature = "wasm", ts(export))]
pub struct FooterReference {
    pub footer_type: String,
    pub id: String,
}

impl FooterReference {
    pub fn new(t: impl Into<String>, id: impl Into<String>) -> FooterReference {
        FooterReference {
            footer_type: t.into(),
            id: id.into(),
        }
    }
}

impl BuildXML for FooterReference {
    fn build(&self) -> Vec<u8> {
        XMLBuilder::new()
            .footer_reference(&self.footer_type, &self.id)
            .build()
    }
}
