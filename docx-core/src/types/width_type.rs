use serde::Serialize;
use std::fmt;
use wasm_bindgen::prelude::*;

use super::errors;
use std::str::FromStr;

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum WidthType {
    Dxa,
    Auto,
    Pct,
    Unsupported,
}

impl fmt::Display for WidthType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WidthType::Dxa => write!(f, "Dxa"),
            WidthType::Auto => write!(f, "auto"),
            WidthType::Pct => write!(f, "pct"),
            WidthType::Unsupported => write!(f, "unsupported"),
        }
    }
}

impl FromStr for WidthType {
    type Err = errors::TypeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Dxa" => Ok(WidthType::Dxa),
            "auto" => Ok(WidthType::Auto),
            "pct" => Ok(WidthType::Pct),
            _ => Ok(WidthType::Unsupported),
        }
    }
}
