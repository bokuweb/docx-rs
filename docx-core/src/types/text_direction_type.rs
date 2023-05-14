use std::fmt;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use serde::Serialize;

use super::errors;
use std::str::FromStr;

// ST_TextDirection defines `lr`, `lrV`, `rl`, `rlV`, `tb`, `tbV`.
// However Microsoft word use `tbRlV`, `tbRl`, `btLr`, `lrTbV`.
#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum TextDirectionType {
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

impl fmt::Display for TextDirectionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TextDirectionType::Lr => write!(f, "lr"),
            TextDirectionType::LrV => write!(f, "lrV"),
            TextDirectionType::Rl => write!(f, "rl"),
            TextDirectionType::RlV => write!(f, "rlV"),
            TextDirectionType::Tb => write!(f, "tb"),
            TextDirectionType::TbV => write!(f, "tbV"),
            TextDirectionType::TbRlV => write!(f, "tbRlV"),
            TextDirectionType::TbRl => write!(f, "tbRl"),
            TextDirectionType::BtLr => write!(f, "btLr"),
            TextDirectionType::LrTbV => write!(f, "lrTbV"),
        }
    }
}

impl FromStr for TextDirectionType {
    type Err = errors::TypeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "lr" => Ok(TextDirectionType::Lr),
            "lrV" => Ok(TextDirectionType::LrV),
            "rl" => Ok(TextDirectionType::Rl),
            "rlV" => Ok(TextDirectionType::RlV),
            "tb" => Ok(TextDirectionType::Tb),
            "tbV" => Ok(TextDirectionType::TbV),
            "tbRlV" => Ok(TextDirectionType::TbRlV),
            "tbRl" => Ok(TextDirectionType::TbRl),
            "btLr" => Ok(TextDirectionType::BtLr),
            "lrTbV" => Ok(TextDirectionType::LrTbV),
            _ => Err(errors::TypeError::FromStrError),
        }
    }
}
