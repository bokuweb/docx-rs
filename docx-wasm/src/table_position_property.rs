use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct TablePositionProperty(docx_rs::TablePositionProperty);

#[wasm_bindgen(js_name = createTablePosition)]
pub fn create_table_position() -> TablePositionProperty {
    TablePositionProperty(docx_rs::TablePositionProperty::new())
}

#[wasm_bindgen]
impl TablePositionProperty {
    pub fn left_from_text(mut self, v: i32) -> Self {
        self.0.left_from_text = Some(v);
        self
    }

    pub fn right_from_text(mut self, v: i32) -> Self {
        self.0.right_from_text = Some(v);
        self
    }

    pub fn vertical_anchor(mut self, v: &str) -> Self {
        self.0.vertical_anchor = Some(v.into());
        self
    }

    pub fn horizontal_anchor(mut self, v: &str) -> Self {
        self.0.horizontal_anchor = Some(v.into());
        self
    }

    pub fn position_x_alignment(mut self, v: &str) -> Self {
        self.0.position_x_alignment = Some(v.into());
        self
    }

    pub fn position_y_alignment(mut self, v: &str) -> Self {
        self.0.position_y_alignment = Some(v.into());
        self
    }

    pub fn position_x(mut self, v: i32) -> Self {
        self.0.position_x = Some(v);
        self
    }

    pub fn position_y(mut self, v: i32) -> Self {
        self.0.position_y = Some(v);
        self
    }
}

impl TablePositionProperty {
    pub fn take(self) -> docx_rs::TablePositionProperty {
        self.0
    }
}
