use wasm_bindgen::prelude::*;
use serde::Serialize;

#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum BorderPosition {
    Left,
    Right,
    Top,
    Bottom,
    InsideH,
    InsideV,
}
