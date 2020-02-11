use std::fmt;
use wasm_bindgen::prelude::*;

use super::errors;
use std::str::FromStr;

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum VMergeType {
    Continue,
    Restart,
}

impl fmt::Display for VMergeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            VMergeType::Continue => write!(f, "continue"),
            VMergeType::Restart => write!(f, "restart"),
        }
    }
}

impl FromStr for VMergeType {
    type Err = errors::TypeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "continue" => Ok(VMergeType::Continue),
            "restart" => Ok(VMergeType::Restart),
            _ => Err(errors::TypeError::FromStrError),
        }
    }
}
