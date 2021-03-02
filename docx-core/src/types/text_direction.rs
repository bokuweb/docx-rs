use std::fmt;
use wasm_bindgen::prelude::*;

use serde::Serialize;

use super::errors;
use std::str::FromStr;

// ST_TextDirection defines `lr`, `lrV`, `rl`, `rlV`, `tb`, `tbV`.
// However Microsoft word use `tbRlV`, `tbRl`, `btLr`, `lrTbV`.
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum TextDirection {
    Lr,
    LrV,
    Rl,
    RlV,
    Tb,
    TbV,
    TbRlV,
    TbRl,
    BtLr,
    LrTbV,
}

impl fmt::Display for TextDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TextDirection::Lr => write!(f, "lr"),
            TextDirection::LrV => write!(f, "lrV"),
            TextDirection::Rl => write!(f, "rl"),
            TextDirection::RlV => write!(f, "rlV"),
            TextDirection::Tb => write!(f, "tb"),
            TextDirection::TbV => write!(f, "tbV"),
            TextDirection::TbRlV => write!(f, "tbRlV"),
            TextDirection::TbRl => write!(f, "tbRl"),
            TextDirection::BtLr => write!(f, "btLr"),
            TextDirection::LrTbV => write!(f, "lrTbV"),
        }
    }
}

impl FromStr for TextDirection {
    type Err = errors::TypeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "lr" => Ok(TextDirection::Lr),
            "lrV" => Ok(TextDirection::LrV),
            "rl" => Ok(TextDirection::Rl),
            "rlV" => Ok(TextDirection::RlV),
            "tb" => Ok(TextDirection::Tb),
            "tbV" => Ok(TextDirection::TbV),
            "tbRlV" => Ok(TextDirection::TbRlV),
            "tbRl" => Ok(TextDirection::TbRl),
            "btLr" => Ok(TextDirection::BtLr),
            "lrTbV" => Ok(TextDirection::LrTbV),
            _ => Err(errors::TypeError::FromStrError),
        }
    }
}
