use std::fmt;
use wasm_bindgen::prelude::*;

use serde::Serialize;

use super::errors;
use std::str::FromStr;

#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum LevelSuffixType {
    Nothing,
    Space,
    Tab,
}

impl fmt::Display for LevelSuffixType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LevelSuffixType::Nothing => write!(f, "nothing"),
            LevelSuffixType::Space => write!(f, "space"),
            LevelSuffixType::Tab => write!(f, "tab"),
        }
    }
}

impl FromStr for LevelSuffixType {
    type Err = errors::TypeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "nothing" => Ok(LevelSuffixType::Nothing),
            "space" => Ok(LevelSuffixType::Space),
            "tab" => Ok(LevelSuffixType::Tab),
            _ => Ok(LevelSuffixType::Tab),
        }
    }
}
