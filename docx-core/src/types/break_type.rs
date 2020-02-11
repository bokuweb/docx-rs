use serde::{Deserialize, Serialize};

//
// Please see <xsd:simpleType name="ST_BrType">
//

use std::fmt;
use std::str::FromStr;
use wasm_bindgen::prelude::*;

use super::errors;

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum BreakType {
    Page,
    Column,
    TextWrapping,
}

impl fmt::Display for BreakType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BreakType::Page => write!(f, "page"),
            BreakType::Column => write!(f, "column"),
            BreakType::TextWrapping => write!(f, "textWrapping"),
        }
    }
}

impl FromStr for BreakType {
    type Err = errors::TypeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "page" => Ok(BreakType::Page),
            "column" => Ok(BreakType::Column),
            "textWrapping" => Ok(BreakType::TextWrapping),
            _ => Err(errors::TypeError::FromStrError),
        }
    }
}
