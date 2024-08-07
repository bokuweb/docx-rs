use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct PositionalTab(docx_rs::PositionalTab);

#[wasm_bindgen(js_name = createPositionalTab)]
pub fn create_positional_tab(
    alignment: docx_rs::PositionalTabAlignmentType,
    relative_to: docx_rs::PositionalTabRelativeTo,
    leader: docx_rs::TabLeaderType,
) -> PositionalTab {
    PositionalTab(docx_rs::PositionalTab::new(alignment, relative_to, leader))
}

#[wasm_bindgen]
impl PositionalTab {
    pub fn alignment(mut self, alignment: docx_rs::PositionalTabAlignmentType) -> Self {
        self.0 = self.0.alignment(alignment);
        self
    }

    pub fn relative_to(mut self, relative_to: docx_rs::PositionalTabRelativeTo) -> Self {
        self.0 = self.0.relative_to(relative_to);
        self
    }

    pub fn leader(mut self, leader: docx_rs::TabLeaderType) -> Self {
        self.0 = self.0.leader(leader);
        self
    }
}

impl PositionalTab {
    pub fn take(self) -> docx_rs::PositionalTab {
        self.0
    }
}
