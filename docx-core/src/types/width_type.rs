use serde::Serialize;
use std::fmt;
use wasm_bindgen::prelude::*;

use super::errors;
use std::str::FromStr;

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, PartialEq, Serialize)]
pub enum WidthType {
    DXA,
    Auto,
    Pct,
    Nil,
    Unsupported,
}

impl fmt::Display for WidthType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WidthType::DXA => write!(f, "dxa"),
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
        match s {
            "dxa" => Ok(WidthType::DXA),
            "auto" => Ok(WidthType::Auto),
            "pct" => Ok(WidthType::Pct),
            "nil" => Ok(WidthType::Nil),
            _ => Ok(WidthType::Unsupported),
        }
    }
}
