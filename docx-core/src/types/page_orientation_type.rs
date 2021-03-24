use serde::{Deserialize, Serialize};

use std::fmt;
use std::str::FromStr;
use wasm_bindgen::prelude::*;

use super::errors;

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum PageOrientationType {
    Landscape,
    Portrait,
}

impl fmt::Display for PageOrientationType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PageOrientationType::Landscape => write!(f, "landscape"),
            PageOrientationType::Portrait => write!(f, "portrait"),
        }
    }
}

impl FromStr for PageOrientationType {
    type Err = errors::TypeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "landscape" => Ok(PageOrientationType::Landscape),
            "portrait" => Ok(PageOrientationType::Portrait),
            _ => Ok(PageOrientationType::Portrait),
        }
    }
}
