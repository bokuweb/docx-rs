use serde::{Deserialize, Serialize};

use std::fmt;
use std::str::FromStr;
use wasm_bindgen::prelude::*;

use super::errors;

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum TabValueType {
    Bar,
    Center,
    Clear,
    Decimal,
    End,
    Right,
    Num,
    Start,
    Left,
}

impl fmt::Display for TabValueType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TabValueType::Bar => write!(f, "bar"),
            TabValueType::Center => write!(f, "center"),
            TabValueType::Clear => write!(f, "clear"),
            TabValueType::Decimal => write!(f, "decimal"),
            TabValueType::End => write!(f, "end"),
            TabValueType::Right => write!(f, "right"),
            TabValueType::Num => write!(f, "num"),
            TabValueType::Start => write!(f, "start"),
            TabValueType::Left => write!(f, "left"),
        }
    }
}

impl FromStr for TabValueType {
    type Err = errors::TypeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bar" => Ok(TabValueType::Bar),
            "center" => Ok(TabValueType::Center),
            "clear" => Ok(TabValueType::Clear),
            "decimal" => Ok(TabValueType::Decimal),
            "end" => Ok(TabValueType::End),
            "right" => Ok(TabValueType::Right),
            "num" => Ok(TabValueType::Num),
            "start" => Ok(TabValueType::Start),
            "left" => Ok(TabValueType::Left),
            _ => Err(errors::TypeError::Unsupported(s.to_string())),
        }
    }
}
