use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct NumPages(docx_rs::NumPages);

#[wasm_bindgen(js_name = createNumPages)]
pub fn create_num_pages() -> NumPages {
    NumPages(docx_rs::NumPages::new())
}

impl NumPages {
    pub fn take(self) -> docx_rs::NumPages {
        self.0
    }
}

#[wasm_bindgen]
impl NumPages {
    pub fn wrap(mut self, wrap: &str) -> Self {
        self.0.frame_property = Some(self.0.frame_property.unwrap_or_default().wrap(wrap));
        self
    }

    pub fn v_anchor(mut self, anchor: &str) -> Self {
        self.0.frame_property = Some(self.0.frame_property.unwrap_or_default().v_anchor(anchor));
        self
    }

    pub fn h_anchor(mut self, anchor: &str) -> Self {
        self.0.frame_property = Some(self.0.frame_property.unwrap_or_default().h_anchor(anchor));
        self
    }

    pub fn h_rule(mut self, r: &str) -> Self {
        self.0.frame_property = Some(self.0.frame_property.unwrap_or_default().h_rule(r));
        self
    }

    pub fn x_align(mut self, align: &str) -> Self {
        self.0.frame_property = Some(self.0.frame_property.unwrap_or_default().x_align(align));
        self
    }

    pub fn y_align(mut self, align: &str) -> Self {
        self.0.frame_property = Some(self.0.frame_property.unwrap_or_default().y_align(align));
        self
    }

    pub fn h_space(mut self, x: i32) -> Self {
        self.0.frame_property = Some(self.0.frame_property.unwrap_or_default().h_space(x));
        self
    }

    pub fn v_space(mut self, x: i32) -> Self {
        self.0.frame_property = Some(self.0.frame_property.unwrap_or_default().v_space(x));
        self
    }

    pub fn x(mut self, x: i32) -> Self {
        self.0.frame_property = Some(self.0.frame_property.unwrap_or_default().x(x));
        self
    }

    pub fn y(mut self, y: i32) -> Self {
        self.0.frame_property = Some(self.0.frame_property.unwrap_or_default().y(y));
        self
    }

    pub fn width(mut self, n: u32) -> Self {
        self.0.frame_property = Some(self.0.frame_property.unwrap_or_default().width(n));
        self
    }

    pub fn height(mut self, n: u32) -> Self {
        self.0.frame_property = Some(self.0.frame_property.unwrap_or_default().height(n));
        self
    }

    // TODO: add other pPr fields
    pub fn align(mut self, alignment_type: docx_rs::AlignmentType) -> Self {
        self.0.paragraph_property = Some(
            self.0
                .paragraph_property
                .unwrap_or_default()
                .align(alignment_type),
        );
        self
    }
}
