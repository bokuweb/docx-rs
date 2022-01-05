use wasm_bindgen::prelude::*;

use super::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct TableOfContents(docx_rs::TableOfContents);

#[wasm_bindgen(js_name = createTableOfContents)]
pub fn create_table_of_contents() -> TableOfContents {
    TableOfContents(docx_rs::TableOfContents::new())
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

    pub fn hyperlink(mut self) -> Self {
        self.0.instr = self.0.instr.hyperlink();
        self
    }

    pub fn alias(mut self, a: &str) -> Self {
        self.0.alias = Some(a.into());
        self
    }

    pub fn add_item(mut self, t: TableOfContentsItem) -> Self {
        self.0.items.push(t.take());
        self
    }

    pub fn disable_auto_items(mut self) -> Self {
        self.0.disable_auto_items = true;
        self
    }

    pub fn dirty(mut self) -> Self {
        self.0.dirty = true;
        self
    }
}
