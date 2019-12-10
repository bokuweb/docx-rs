use super::*;
use docx_core;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Run(docx_core::Run);

#[wasm_bindgen(js_name = createRun)]
pub fn create_run() -> Run {
    Run(docx_core::Run::new())
}

#[wasm_bindgen]
impl Run {
    pub fn add_text(mut self, text: &str) -> Self {
        self.0 = self.0.add_text(text);
        self
    }

    pub fn add_delete_text(mut self, text: String) -> Run {
        self.0
            .children
            .push(docx_core::RunChild::Text(docx_core::Text::new(text)));
        self
    }

    pub fn add_tab(mut self) -> Run {
        self.0
            .children
            .push(docx_core::RunChild::Tab(docx_core::Tab::new()));
        self
    }

    pub fn add_break(mut self, break_type: docx_core::BreakType) -> Run {
        self.0
            .children
            .push(docx_core::RunChild::Break(docx_core::Break::new(
                break_type,
            )));
        self
    }

    pub fn size(mut self, size: usize) -> Run {
        self.0.run_property = self.0.run_property.size(size);
        self
    }

    pub fn color(mut self, color: String) -> Run {
        self.0.run_property = self.0.run_property.color(color);
        self
    }

    pub fn highlight(mut self, color: String) -> Run {
        self.0.run_property = self.0.run_property.highlight(color);
        self
    }

    pub fn bold(mut self) -> Run {
        self.0.run_property = self.0.run_property.bold();
        self
    }

    pub fn italic(mut self) -> Run {
        self.0.run_property = self.0.run_property.italic();
        self
    }

    pub fn underline(mut self, line_type: String) -> Run {
        self.0.run_property = self.0.run_property.underline(line_type);
        self
    }
}

impl Run {
    pub fn take(self) -> docx_core::Run {
        self.0
    }
}
