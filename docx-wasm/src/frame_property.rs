use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct FrameProperty(docx_rs::FrameProperty);

#[wasm_bindgen(js_name = createFrameProperty)]
pub fn create_frame_property() -> FrameProperty {
    FrameProperty(docx_rs::FrameProperty::new())
}

#[wasm_bindgen]
impl FrameProperty {
    // frame property
    pub fn wrap(mut self, wrap: &str) -> Self {
        self.0 = self.0.wrap(wrap);
        self
    }

    pub fn v_anchor(mut self, anchor: &str) -> Self {
        self.0 = self.0.v_anchor(anchor);
        self
    }

    pub fn h_anchor(mut self, anchor: &str) -> Self {
        self.0 = self.0.h_anchor(anchor);
        self
    }

    pub fn h_rule(mut self, r: &str) -> Self {
        self.0 = self.0.h_rule(r);
        self
    }

    pub fn x_align(mut self, align: &str) -> Self {
        self.0 = self.0.x_align(align);
        self
    }

    pub fn y_align(mut self, align: &str) -> Self {
        self.0 = self.0.y_align(align);
        self
    }

    pub fn h_space(mut self, x: i32) -> Self {
        self.0 = self.0.h_space(x);
        self
    }

    pub fn v_space(mut self, x: i32) -> Self {
        self.0 = self.0.v_space(x);
        self
    }

    pub fn x(mut self, x: i32) -> Self {
        self.0 = self.0.x(x);
        self
    }

    pub fn y(mut self, y: i32) -> Self {
        self.0 = self.0.y(y);
        self
    }

    pub fn width(mut self, n: u32) -> Self {
        self.0 = self.0.width(n);
        self
    }

    pub fn height(mut self, n: u32) -> Self {
        self.0 = self.0.height(n);
        self
    }
}

impl FrameProperty {
    pub fn take(self) -> docx_rs::FrameProperty {
        self.0
    }
}
