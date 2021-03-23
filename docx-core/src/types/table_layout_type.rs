use serde::{Deserialize, Serialize};
use std::fmt;
use wasm_bindgen::prelude::*;

use super::errors;
use std::str::FromStr;

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TableLayoutType {
    Fixed,
    Autofit,
}

impl fmt::Display for TableLayoutType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TableLayoutType::Fixed => write!(f, "fixed"),
            TableLayoutType::Autofit => write!(f, "autofit"),
        }
    }
}

impl FromStr for TableLayoutType {
    type Err = errors::TypeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "fixed" => Ok(TableLayoutType::Fixed),
            _ => Ok(TableLayoutType::Autofit),
        }
    }
}
