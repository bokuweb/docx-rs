use serde::Serialize;
use std::fmt;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Debug, Clone, Copy, Serialize, PartialEq)]
#[cfg_attr(feature = "wasm", derive(ts_rs::TS))]
#[cfg_attr(feature = "wasm", ts(export))]
#[serde(rename_all = "camelCase")]
pub enum DrawingPositionType {
    Anchor,
    Inline,
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Debug, Clone, Copy, Serialize, PartialEq)]
#[cfg_attr(feature = "wasm", derive(ts_rs::TS))]
#[cfg_attr(feature = "wasm", ts(export))]
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

#[derive(Debug, Clone, Copy, Serialize, PartialEq)]
#[cfg_attr(feature = "wasm", derive(ts_rs::TS))]
#[cfg_attr(feature = "wasm", ts(export))]
#[serde(rename_all = "camelCase")]
pub enum DrawingPosition {
    Offset(i32),
    Align(PicAlign),
}
