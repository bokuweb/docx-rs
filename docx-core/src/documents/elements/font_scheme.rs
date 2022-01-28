use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize, ts_rs::TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct FontSchemeFont {
    pub script: String,
    pub typeface: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Default, ts_rs::TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct FontGroup {
    pub latin: String,
    pub ea: String,
    pub cs: String,
    pub fonts: Vec<FontSchemeFont>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Default, ts_rs::TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct FontScheme {
    pub major_font: FontGroup,
    pub minor_font: FontGroup,
}

// For now reader only
impl FontScheme {
    pub fn new() -> Self {
        Self::default()
    }
}
