use serde::Serialize;
use std::fmt;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, Serialize, PartialEq, ts_rs::TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub enum DrawingPositionType {
    Anchor,
    Inline,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, Serialize, PartialEq, ts_rs::TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub enum PicAlign {
    Left,
    Right,
    Center,
    Bottom,
    Top,
}

impl fmt::Display for PicAlign {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PicAlign::Left => write!(f, "left"),
            PicAlign::Right => write!(f, "right"),
            PicAlign::Center => write!(f, "center"),
            PicAlign::Bottom => write!(f, "bottom"),
            PicAlign::Top => write!(f, "top"),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, ts_rs::TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub enum DrawingPosition {
    Offset(i32),
    Align(PicAlign),
}
