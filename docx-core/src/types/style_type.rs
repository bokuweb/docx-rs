use std::fmt;
use wasm_bindgen::prelude::*;

use serde::Serialize;

#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq, Serialize, Copy)]
#[serde(rename_all = "camelCase")]
pub enum StyleType {
    Paragraph,
    Character,
}

impl fmt::Display for StyleType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StyleType::Paragraph => write!(f, "paragraph"),
            StyleType::Character => write!(f, "character"),
        }
    }
}
