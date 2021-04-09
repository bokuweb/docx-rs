use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Div {
    pub id: String,
    pub margin_left: usize,
    pub margin_right: usize,
    pub margin_top: usize,
    pub margin_bottom: usize,
    pub divs_child: Vec<Div>,
}

impl Default for Div {
    fn default() -> Self {
        Self {
            id: "".to_string(),
            margin_left: 0,
            margin_right: 0,
            margin_top: 0,
            margin_bottom: 0,
            divs_child: vec![],
        }
    }
}

impl Div {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            ..Default::default()
        }
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

    pub fn add_child(mut self, s: Div) -> Self {
        self.divs_child.push(s);
        self
    }
}
