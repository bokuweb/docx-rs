use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Div {
    pub margin_left: usize,
    pub margin_right: usize,
    pub margin_top: usize,
    pub margin_bottom: usize,
}

impl Default for Div {
    fn default() -> Self {
        Self {
            margin_left: 0,
            margin_right: 0,
            margin_top: 0,
            margin_bottom: 0,
        }
    }
}

impl Div {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn margin_left(mut self, s: usize) -> Self {
        self.margin_left = s;
        self
    }

    pub fn margin_right(mut self, s: usize) -> Self {
        self.margin_right = s;
        self
    }

    pub fn margin_top(mut self, s: usize) -> Self {
        self.margin_top = s;
        self
    }

    pub fn margin_bottom(mut self, s: usize) -> Self {
        self.margin_bottom = s;
        self
    }
}
