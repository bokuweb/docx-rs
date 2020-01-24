use super::*;
use docx;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Run(docx::Run);

#[wasm_bindgen(js_name = createRun)]
pub fn create_run() -> Run {
    Run(docx::Run::new())
}

#[wasm_bindgen]
impl Run {
    pub fn add_text(mut self, text: &str) -> Self {
        self.0 = self.0.add_text(text);
        self
    }

    pub fn add_delete_text(mut self, text: &str) -> Run {
        self.0
            .children
            .push(docx::RunChild::DeleteText(docx::DeleteText::new(text)));
        self
    }

    pub fn add_tab(mut self) -> Run {
        self.0.children.push(docx::RunChild::Tab(docx::Tab::new()));
        self
    }

    pub fn add_break(mut self, break_type: docx::BreakType) -> Run {
        self.0
            .children
            .push(docx::RunChild::Break(docx::Break::new(break_type)));
        self
    }

    pub fn size(mut self, size: usize) -> Run {
        self.0.run_property = self.0.run_property.size(size);
        self
    }

    pub fn color(mut self, color: &str) -> Run {
        self.0.run_property = self.0.run_property.color(color);
        self
    }

    pub fn highlight(mut self, color: &str) -> Run {
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

    pub fn underline(mut self, line_type: &str) -> Run {
        self.0.run_property = self.0.run_property.underline(line_type);
        self
    }

    pub fn vanish(mut self) -> Run {
        self.0.run_property = self.0.run_property.vanish();
        self
    }
}

impl Run {
    pub fn take(self) -> docx::Run {
        self.0
    }
}
