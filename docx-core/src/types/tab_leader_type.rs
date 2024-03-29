use serde::{Deserialize, Serialize};

use std::fmt;
use std::str::FromStr;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use super::errors;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[cfg_attr(feature = "wasm", derive(ts_rs::TS))]
#[cfg_attr(feature = "wasm", ts(export))]
#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum TabLeaderType {
    Dot,
    Heavy,
    Hyphen,
    MiddleDot,
    None,
    Underscore,
}

impl fmt::Display for TabLeaderType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TabLeaderType::Dot => write!(f, "dot"),
            TabLeaderType::Heavy => write!(f, "heavy"),
            TabLeaderType::Hyphen => write!(f, "hyphen"),
            TabLeaderType::MiddleDot => write!(f, "middleDot"),
            TabLeaderType::None => write!(f, "none"),
            TabLeaderType::Underscore => write!(f, "underscore"),
        }
    }
}

impl FromStr for TabLeaderType {
    type Err = errors::TypeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "dot" => Ok(TabLeaderType::Dot),
            "heavy" => Ok(TabLeaderType::Heavy),
            "hyphen" => Ok(TabLeaderType::Hyphen),
            "middleDot" => Ok(TabLeaderType::MiddleDot),
            "none" => Ok(TabLeaderType::None),
            "underscore" => Ok(TabLeaderType::Underscore),
            _ => Err(errors::TypeError::Unsupported(s.to_string())),
        }
    }
}
