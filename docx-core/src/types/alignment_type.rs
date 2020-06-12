use std::fmt;
use std::str::FromStr;
use wasm_bindgen::prelude::*;

use super::errors;

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub enum AlignmentType {
    Both,
    Center,
    Distribute,
    End,
    Left,
    Right,
    Justified,
    Unsupported,
}

impl fmt::Display for AlignmentType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AlignmentType::Center => write!(f, "center"),
            AlignmentType::Left => write!(f, "left"),
            AlignmentType::Distribute => write!(f, "distribute"),
            AlignmentType::Right => write!(f, "right"),
            AlignmentType::End => write!(f, "end"),
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
            "distribute" => Ok(AlignmentType::Distribute),
            "center" => Ok(AlignmentType::Center),
            "both" => Ok(AlignmentType::Both),
            "end" => Ok(AlignmentType::End),
            "justified" => Ok(AlignmentType::Justified),
            _ => Ok(AlignmentType::Unsupported),
        }
    }
}
