use std::fmt;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub enum TableAlignmentType {
    Center,
    Left,
    Right,
}

impl fmt::Display for TableAlignmentType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TableAlignmentType::Center => write!(f, "center"),
            TableAlignmentType::Left => write!(f, "left"),
            TableAlignmentType::Right => write!(f, "right"),
        }
    }
}
