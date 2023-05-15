use std::fmt;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use super::errors;
use std::str::FromStr;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum VMergeType {
    Continue,
    Restart,
    Unsupported,
}

impl fmt::Display for VMergeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            VMergeType::Continue => write!(f, "continue"),
            VMergeType::Restart => write!(f, "restart"),
            VMergeType::Unsupported => write!(f, "unsupported"),
        }
    }
}

impl FromStr for VMergeType {
    type Err = errors::TypeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "continue" => Ok(VMergeType::Continue),
            "restart" => Ok(VMergeType::Restart),
            _ => Ok(VMergeType::Unsupported),
        }
    }
}
