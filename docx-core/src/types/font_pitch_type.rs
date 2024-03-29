use std::fmt;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Copy, Clone, Debug)]
pub enum FontPitchType {
    Default,
    Fixed,
    Variable,
}

impl fmt::Display for FontPitchType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FontPitchType::Default => write!(f, "default"),
            FontPitchType::Fixed => write!(f, "fixed"),
            FontPitchType::Variable => write!(f, "variable"),
        }
    }
}
