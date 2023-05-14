use std::fmt;
use std::str::FromStr;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use super::errors;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Copy, Clone, Debug)]
pub enum TableAlignmentType {
    Center,
    Left,
    Right,
}

impl fmt::Display for TableAlignmentType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TableAlignmentType::Center => write!(f, "center"),
            TableAlignmentType::Left => write!(f, "left"),
            TableAlignmentType::Right => write!(f, "right"),
        }
    }
}

impl FromStr for TableAlignmentType {
    type Err = errors::TypeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "start" | "left" => Ok(TableAlignmentType::Left),
            "right" | "end" => Ok(TableAlignmentType::Right),
            "center" => Ok(TableAlignmentType::Center),
            _ => Ok(TableAlignmentType::Left),
        }
    }
}
