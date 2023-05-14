use std::fmt;
use std::str::FromStr;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use super::errors;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum VertAlignType {
    Baseline,
    SuperScript,
    SubScript,
    Unsupported,
}

// <xsd:enumeration value="baseline"/>
// <xsd:enumeration value="superscript"/>
// <xsd:enumeration value="subscript"/>
impl fmt::Display for VertAlignType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            VertAlignType::SuperScript => write!(f, "superscript"),
            VertAlignType::SubScript => write!(f, "subscript"),
            VertAlignType::Baseline => write!(f, "baseline"),
            _ => write!(f, "unsupported"),
        }
    }
}

impl FromStr for VertAlignType {
    type Err = errors::TypeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "subscript" => Ok(VertAlignType::SubScript),
            "superscript" => Ok(VertAlignType::SuperScript),
            "baseline" => Ok(VertAlignType::Baseline),
            _ => Ok(VertAlignType::Unsupported),
        }
    }
}
