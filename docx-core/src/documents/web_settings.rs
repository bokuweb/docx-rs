use serde::Serialize;

use super::*;

#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct WebSettings {
    pub divs: Vec<Div>,
}

impl WebSettings {
    pub fn new() -> WebSettings {
        Default::default()
    }
}
