use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Run(docx_rs::Run);

#[wasm_bindgen(js_name = createRun)]
pub fn create_run() -> Run {
    Run(docx_rs::Run::new())
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
            .push(docx_rs::RunChild::DeleteText(docx_rs::DeleteText::new(
                text,
            )));
        self
    }

    pub fn add_tab(mut self) -> Run {
        self.0
            .children
            .push(docx_rs::RunChild::Tab(docx_rs::Tab::new()));
        self
    }

    pub fn add_break(mut self, break_type: docx_rs::BreakType) -> Run {
        self.0
            .children
            .push(docx_rs::RunChild::Break(docx_rs::Break::new(break_type)));
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

    pub fn fonts(mut self, f: RunFonts) -> Self {
        self.0 = self.0.fonts(f.take());
        self
    }

    pub fn spacing(mut self, spacing: u32) -> Run {
        self.0.run_property = self.0.run_property.spacing(spacing);
        self
    }
}

impl Run {
    pub fn take(self) -> docx_rs::Run {
        self.0
    }
}
