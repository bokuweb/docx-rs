use serde::Serialize;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum TableBorderPosition {
    Left,
    Right,
    Top,
    Bottom,
    InsideH,
    InsideV,
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum TableCellBorderPosition {
    Left,
    Right,
    Top,
    Bottom,
    InsideH,
    InsideV,
    Tl2br,
    Tr2bl,
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ParagraphBorderPosition {
    Left,
    Right,
    Top,
    Bottom,
    Between,
    Bar,
}
