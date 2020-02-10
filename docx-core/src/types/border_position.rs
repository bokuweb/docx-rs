use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum BorderPosition {
    Left,
    Right,
    Top,
    Bottom,
    IndideH,
    IndideV,
}
