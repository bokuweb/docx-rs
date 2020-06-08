use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct RunFonts(docx_rs::RunFonts);

#[wasm_bindgen(js_name = createRunFonts)]
pub fn create_run_fonts() -> RunFonts {
    RunFonts(docx_rs::RunFonts::new())
}

impl RunFonts {
    pub fn take(self) -> docx_rs::RunFonts {
        self.0
    }
}

#[wasm_bindgen]
impl RunFonts {
    pub fn ascii(mut self, f: String) -> Self {
        self.0 = self.0.ascii(f);
        self
    }

    pub fn hi_ansi(mut self, f: String) -> Self {
        self.0 = self.0.hi_ansi(f);
        self
    }

    pub fn cs(mut self, f: String) -> Self {
        self.0 = self.0.cs(f);
        self
    }

    pub fn east_asia(mut self, f: String) -> Self {
        self.0 = self.0.east_asia(f);
        self
    }
}
