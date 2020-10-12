use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PageMargin {
    pub top: u32,
    pub left: u32,
    pub bottom: u32,
    pub right: u32,
    pub header: u32,
    pub footer: u32,
    pub gutter: u32,
}
