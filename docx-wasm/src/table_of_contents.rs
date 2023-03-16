use wasm_bindgen::prelude::*;

use super::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct TableOfContents(docx_rs::TableOfContents);

#[wasm_bindgen(js_name = createTableOfContents)]
pub fn create_table_of_contents() -> TableOfContents {
    TableOfContents(docx_rs::TableOfContents::new())
}

#[wasm_bindgen(js_name = createTableOfContentsWithInstrText)]
pub fn create_table_of_contents_with_instr_text(s: &str) -> TableOfContents {
    TableOfContents(docx_rs::TableOfContents::with_instr_text(s))
}

impl TableOfContents {
    pub fn take(self) -> docx_rs::TableOfContents {
        self.0
    }
}

#[wasm_bindgen]
impl TableOfContents {
    pub fn heading_styles_range(mut self, start: usize, end: usize) -> Self {
        self.0.instr = self.0.instr.heading_styles_range(start, end);
        self
    }

    pub fn add_style_with_level(mut self, style: &str, level: usize) -> Self {
        self.0.instr = self
            .0
            .instr
            .add_style_with_level(docx_rs::StyleWithLevel::new(style, level));
        self
    }

    pub fn hyperlink(mut self) -> Self {
        self.0.instr = self.0.instr.hyperlink();
        self
    }

    pub fn alias(mut self, a: &str) -> Self {
        self.0.alias = Some(a.into());
        self
    }

    pub fn page_ref_placeholder(mut self, a: &str) -> Self {
        self.0.page_ref_placeholder = Some(a.into());
        self
    }

    pub fn add_item(mut self, t: TableOfContentsItem) -> Self {
        self.0.items.push(t.take());
        self
    }

    pub fn auto(mut self) -> Self {
        self.0.auto = true;
        self
    }

    pub fn dirty(mut self) -> Self {
        self.0.dirty = true;
        self
    }

    pub fn without_sdt(mut self) -> Self {
        self.0.without_sdt = true;
        self
    }

    pub fn delete(mut self, author: &str, date: &str) -> Self {
        self.0 = self.0.delete(author, date);
        self
    }

    pub fn add_before_paragraph(mut self, p: Paragraph) -> Self {
        self.0 = self.0.add_before_paragraph(p.take());
        self
    }

    pub fn add_after_paragraph(mut self, p: Paragraph) -> Self {
        self.0 = self.0.add_after_paragraph(p.take());
        self
    }

    pub fn add_before_table(mut self, t: Table) -> Self {
        self.0 = self.0.add_before_table(t.take());
        self
    }

    pub fn add_after_table(mut self, t: Table) -> Self {
        self.0 = self.0.add_after_table(t.take());
        self
    }
}
