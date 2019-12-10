use super::*;
use docx_core;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct TableCell(docx_core::TableCell);

#[wasm_bindgen(js_name = createTableCell)]
pub fn create_table_cell() -> TableCell {
    TableCell(docx_core::TableCell::new())
}

impl TableCell {
    pub fn take(self) -> docx_core::TableCell {
        self.0
    }

    pub fn add_paragraph(mut self, p: Paragraph) -> TableCell {
        self.0
            .contents
            .push(docx_core::TableCellContent::Paragraph(p.take()));
        self
    }

    pub fn vertical_merge(mut self, t: docx_core::VMergeType) -> TableCell {
        self.0.property = self.0.property.vertical_merge(t);
        self
    }

    pub fn grid_span(mut self, v: usize) -> TableCell {
        self.0.property = self.0.property.grid_span(v);
        self
    }
}

#[wasm_bindgen]
impl TableCell {}
