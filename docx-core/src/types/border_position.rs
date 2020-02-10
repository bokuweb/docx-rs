use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum BorderPosition {
    Left,
    Right,
    Top,
    Bottom,
    IndideH,
    IndideV,
}
