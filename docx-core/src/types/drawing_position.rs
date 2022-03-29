use serde::Serialize;

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
    Bottom,
    Top,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, ts_rs::TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub enum DrawingPosition {
    Offset(i32),
    Align(PicAlign),
}
