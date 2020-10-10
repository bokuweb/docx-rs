use serde::Serialize;

#[derive(Copy, Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SpacingType {
    Value(u32),
    Line(u32),
}
