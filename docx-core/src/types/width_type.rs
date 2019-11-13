use std::fmt;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub enum WidthType {
    DXA,
    AUTO,
}

impl fmt::Display for WidthType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WidthType::DXA => write!(f, "dxa"),
            WidthType::AUTO => write!(f, "auto"),
        }
    }
}
