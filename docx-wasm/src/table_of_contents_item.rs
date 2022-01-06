use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct TableOfContentsItem(docx_rs::TableOfContentsItem);

#[wasm_bindgen(js_name = createTableOfContentsItem)]
pub fn create_table_of_contents_item() -> TableOfContentsItem {
    TableOfContentsItem(docx_rs::TableOfContentsItem::new())
}

impl TableOfContentsItem {
    pub fn take(self) -> docx_rs::TableOfContentsItem {
        self.0
    }
}

#[wasm_bindgen]
impl TableOfContentsItem {
    pub fn text(mut self, a: &str) -> Self {
        self.0.text = a.into();
        self
    }

    pub fn toc_key(mut self, a: &str) -> Self {
        self.0.toc_key = a.into();
        self
    }

    pub fn level(mut self, l: usize) -> Self {
        self.0.level = l;
        self
    }

    pub fn page_ref(mut self, a: &str) -> Self {
        self.0.page_ref = Some(a.into());
        self
    }
}
