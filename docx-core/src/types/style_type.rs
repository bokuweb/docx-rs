use std::fmt;
use wasm_bindgen::prelude::*;

use super::errors;
use serde::Serialize;
use std::str::FromStr;

#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq, Serialize, Copy)]
#[serde(rename_all = "camelCase")]
pub enum StyleType {
    Paragraph,
    Character,
    Numbering,
    Table,
    Unsupported,
}

impl fmt::Display for StyleType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StyleType::Paragraph => write!(f, "paragraph"),
            StyleType::Character => write!(f, "character"),
            StyleType::Numbering => write!(f, "numbering"),
            StyleType::Table => write!(f, "table"),
            StyleType::Unsupported => write!(f, "unsupported"),
        }
    }
}

impl FromStr for StyleType {
    type Err = errors::TypeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "paragraph" => Ok(StyleType::Paragraph),
            "character" => Ok(StyleType::Character),
            "numbering" => Ok(StyleType::Numbering),
            "table" => Ok(StyleType::Table),
            _ => Ok(StyleType::Unsupported),
        }
    }
}
