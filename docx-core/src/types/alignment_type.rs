use std::fmt;
use std::str::FromStr;
use wasm_bindgen::prelude::*;

use super::errors;

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub enum AlignmentType {
    Center,
    Left,
    Right,
    Both,
    Justified,
    Unsupported,
}

impl fmt::Display for AlignmentType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AlignmentType::Center => write!(f, "center"),
            AlignmentType::Left => write!(f, "left"),
            AlignmentType::Right => write!(f, "right"),
            AlignmentType::Both => write!(f, "both"),
            AlignmentType::Justified => write!(f, "justified"),
            _ => write!(f, "unsupported"),
        }
    }
}

impl FromStr for AlignmentType {
    type Err = errors::TypeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "left" => Ok(AlignmentType::Left),
            "right" => Ok(AlignmentType::Right),
            "center" => Ok(AlignmentType::Center),
            "both" => Ok(AlignmentType::Both),
            "justified" => Ok(AlignmentType::Justified),
            _ => Ok(AlignmentType::Unsupported),
        }
    }
}
