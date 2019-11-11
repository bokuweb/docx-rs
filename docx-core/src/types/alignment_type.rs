use std::fmt;
use wasm_bindgen::prelude::*;

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
