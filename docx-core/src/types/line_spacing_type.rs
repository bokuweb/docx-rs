use crate::types::errors;
use crate::TypeError;
use serde::*;
use std::str::FromStr;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LineSpacingType {
    Auto,
    AtLeast,
    Exact,
}

impl FromStr for LineSpacingType {
    type Err = errors::TypeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto" => Ok(LineSpacingType::Auto),
            "atLeast" => Ok(LineSpacingType::AtLeast),
            "exact" => Ok(LineSpacingType::Exact),
            _ => Err(TypeError::FromStrError),
        }
    }
}
