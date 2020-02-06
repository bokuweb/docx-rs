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
    Justified,
}

impl fmt::Display for AlignmentType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AlignmentType::Center => write!(f, "center"),
            AlignmentType::Left => write!(f, "left"),
            AlignmentType::Right => write!(f, "right"),
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
            "center" => Ok(AlignmentType::Center),
            "justified" => Ok(AlignmentType::Justified),
            _ => Err(errors::TypeError::FromStrError),
        }
    }
}
