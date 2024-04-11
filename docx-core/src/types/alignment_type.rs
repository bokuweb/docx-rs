use serde::Serialize;
use std::fmt;
use std::str::FromStr;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use super::errors;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[cfg_attr(feature = "wasm", derive(ts_rs::TS))]
#[cfg_attr(feature = "wasm", ts(export))]
#[derive(Copy, Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum AlignmentType {
    Both,
    Center,
    Distribute,
    Start,
    End,
    Left,
    Right,
    Justified,
}

impl fmt::Display for AlignmentType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AlignmentType::Center => write!(f, "center"),
            AlignmentType::Left => write!(f, "left"),
            AlignmentType::Distribute => write!(f, "distribute"),
            AlignmentType::Right => write!(f, "right"),
            AlignmentType::Start => write!(f, "start"),
            AlignmentType::End => write!(f, "end"),
            AlignmentType::Both => write!(f, "both"),
            AlignmentType::Justified => write!(f, "justified"),
        }
    }
}

impl FromStr for AlignmentType {
    type Err = errors::TypeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "left" => Ok(AlignmentType::Left),
            "right" => Ok(AlignmentType::Right),
            "distribute" => Ok(AlignmentType::Distribute),
            "center" => Ok(AlignmentType::Center),
            "both" => Ok(AlignmentType::Both),
            "start" => Ok(AlignmentType::Start),
            "end" => Ok(AlignmentType::End),
            "justified" => Ok(AlignmentType::Justified),
            _ => Ok(AlignmentType::Left),
        }
    }
}
