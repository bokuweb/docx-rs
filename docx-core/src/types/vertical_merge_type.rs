use std::fmt;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub enum VMergeType {
    Continue,
    Restart,
}

impl fmt::Display for VMergeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            VMergeType::Continue => write!(f, "continue"),
            VMergeType::Restart => write!(f, "restart"),
        }
    }
}
