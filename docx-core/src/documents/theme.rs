use serde::Serialize;

use super::*;

#[derive(Debug, Clone, PartialEq, Serialize, Default, ts_rs::TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct Theme {
    pub font_schema: FontScheme,
}
