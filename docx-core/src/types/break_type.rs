//
// Please see <xsd:simpleType name="ST_BrType">
//

use std::fmt;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub enum BreakType {
    Page,
    Column,
    TextWrapping,
}

impl fmt::Display for BreakType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BreakType::Page => write!(f, "page"),
            BreakType::Column => write!(f, "column"),
            BreakType::TextWrapping => write!(f, "textWrapping"),
        }
    }
}
