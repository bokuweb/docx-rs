use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct LineSpacing(docx_rs::LineSpacing);

#[wasm_bindgen(js_name = createLineSpacing)]
pub fn create_line_spacing() -> LineSpacing {
    LineSpacing(docx_rs::LineSpacing::new())
}

impl LineSpacing {
    pub fn take(self) -> docx_rs::LineSpacing {
        self.0
    }
}

#[wasm_bindgen]
impl LineSpacing {
    pub fn line_rule(mut self, t: docx_rs::LineSpacingType) -> Self {
        self.0 = self.0.line_rule(t);
        self
    }

    pub fn before(mut self, before: u32) -> Self {
        self.0 = self.0.before(before);
        self
    }

    pub fn after(mut self, after: u32) -> Self {
        self.0 = self.0.after(after);
        self
    }

    pub fn before_lines(mut self, before: u32) -> Self {
        self.0 = self.0.before_lines(before);
        self
    }

    pub fn after_lines(mut self, after: u32) -> Self {
        self.0 = self.0.after_lines(after);
        self
    }

    pub fn line(mut self, line: u32) -> Self {
        self.0 = self.0.line(line as i32);
        self
    }
}
