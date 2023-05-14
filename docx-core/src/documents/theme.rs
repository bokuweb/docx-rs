use serde::Serialize;

use super::*;

#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[cfg_attr(feature = "wasm", derive(ts_rs::TS))]
#[cfg_attr(feature = "wasm", ts(export))]
#[serde(rename_all = "camelCase")]
pub struct Theme {
    pub font_schema: FontScheme,
}
