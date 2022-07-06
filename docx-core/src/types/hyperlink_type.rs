use std::fmt;
use wasm_bindgen::prelude::*;

use serde::Serialize;

use super::errors;
use std::str::FromStr;

#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq, Serialize, ts_rs::TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub enum HyperlinkType {
    Anchor,
    External,
}

impl fmt::Display for HyperlinkType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HyperlinkType::Anchor => write!(f, "anchor"),
            HyperlinkType::External => write!(f, "external"),
        }
    }
}

impl FromStr for HyperlinkType {
    type Err = errors::TypeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "anchor" => Ok(HyperlinkType::Anchor),
            "external" => Ok(HyperlinkType::External),
            _ => Ok(HyperlinkType::Anchor),
        }
    }
}
