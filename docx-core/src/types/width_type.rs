use serde::Serialize;
use std::fmt;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use super::errors;
use std::str::FromStr;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Copy, Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum WidthType {
    Dxa,
    Auto,
    Pct,
    Nil,
    Unsupported,
}

impl fmt::Display for WidthType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WidthType::Dxa => write!(f, "dxa"),
            WidthType::Auto => write!(f, "auto"),
            WidthType::Pct => write!(f, "pct"),
            WidthType::Nil => write!(f, "nil"),
            WidthType::Unsupported => write!(f, "unsupported"),
        }
    }
}

impl FromStr for WidthType {
    type Err = errors::TypeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // https://github.com/bokuweb/docx-rs/issues/451
        match s {
            "DXA" | "dxa" => Ok(WidthType::Dxa),
            "Auto" | "auto" => Ok(WidthType::Auto),
            "Pct" | "pct" => Ok(WidthType::Pct),
            "Nil" | "nil" => Ok(WidthType::Nil),
            _ => Ok(WidthType::Unsupported),
        }
    }
}
