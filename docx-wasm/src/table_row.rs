use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct TableRow(docx_rs::TableRow);

#[wasm_bindgen(js_name = createTableRow)]
pub fn create_table_row() -> TableRow {
    TableRow(docx_rs::TableRow::new(vec![]))
}

impl TableRow {
    pub fn take(self) -> docx_rs::TableRow {
        self.0
    }
}

#[wasm_bindgen]
impl TableRow {
    pub fn add_cell(mut self, cell: TableCell) -> TableRow {
        self.0.cells.push(cell.take());
        self
    }

    pub fn row_height(mut self, h: u32) -> TableRow {
        self.0 = self.0.row_height(h as f32);
        self
    }

    pub fn height_rule(mut self, r: docx_rs::HeightRule) -> TableRow {
        self.0 = self.0.height_rule(r);
        self
    }
}
