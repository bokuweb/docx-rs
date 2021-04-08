use serde::Serialize;

use super::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WebSettings {
    pub divs: Vec<Div>,
}

impl WebSettings {
    pub fn new() -> WebSettings {
        Default::default()
    }
}

impl Default for WebSettings {
    fn default() -> Self {
        Self { divs: vec![] }
    }
}
