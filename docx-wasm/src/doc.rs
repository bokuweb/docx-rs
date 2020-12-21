use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Docx(docx_rs::Docx);

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn createDocx() -> Docx {
    Docx(docx_rs::Docx::new())
}

#[wasm_bindgen]
impl Docx {
    pub fn add_paragraph(mut self, p: Paragraph) -> Self {
        self.0 = self.0.add_paragraph(p.take());
        self
    }

    pub fn add_bookmark_start(mut self, id: usize, name: &str) -> Self {
        self.0 = self.0.add_bookmark_start(id, name);
        self
    }

    pub fn add_bookmark_end(mut self, id: usize) -> Docx {
        self.0 = self.0.add_bookmark_end(id);
        self
    }

    pub fn add_table(mut self, t: Table) -> Docx {
        self.0.document = self.0.document.add_table(t.take());
        self
    }

    pub fn add_abstract_numbering(mut self, num: AbstractNumbering) -> Docx {
        self.0.numberings = self.0.numberings.add_abstract_numbering(num.take());
        self
    }

    pub fn add_numbering(mut self, num: Numbering) -> Docx {
        self.0.numberings = self.0.numberings.add_numbering(num.take());
        self
    }

    pub fn doc_id(mut self, id: &str) -> Docx {
        self.0 = self.0.doc_id(id);
        self
    }

    pub fn add_doc_var(mut self, name: &str, val: &str) -> Docx {
        self.0 = self.0.add_doc_var(name, val);
        self
    }

    pub fn page_size(mut self, w: u32, h: u32) -> Docx {
        self.0 = self.0.page_size(w, h);
        self
    }

    pub fn page_margin(mut self, margin: PageMargin) -> Docx {
        self.0 = self.0.page_margin(margin.take());
        self
    }

    pub fn default_size(mut self, size: usize) -> Self {
        self.0.styles = self.0.styles.default_size(size);
        self
    }

    pub fn default_spacing(mut self, spacing: i32) -> Self {
        self.0.styles = self.0.styles.default_spacing(spacing);
        self
    }

    pub fn default_fonts(mut self, font: RunFonts) -> Self {
        self.0.styles = self.0.styles.default_fonts(font.take());
        self
    }

    pub fn build(&mut self, has_numberings: bool) -> Result<Vec<u8>, JsValue> {
        let buf = Vec::new();
        let mut cur = std::io::Cursor::new(buf);
        if has_numberings {
            self.0.document_rels.has_numberings = true;
        }
        let res = self.0.build().pack(&mut cur);
        if res.is_err() {
            return Err(format!("{:?}", res).into());
        }
        Ok(cur.into_inner())
    }

    pub fn json(&self) -> String {
        self.0.json()
    }
}
