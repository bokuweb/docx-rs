use super::*;
use docx;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct TableRow(docx::TableRow);

#[wasm_bindgen(js_name = createTableRow)]
pub fn create_table_row() -> TableRow {
    TableRow(docx::TableRow::new(vec![]))
}

impl TableRow {
    pub fn take(self) -> docx::TableRow {
        self.0
    }
}

#[wasm_bindgen]
impl TableRow {
    pub fn add_cell(mut self, cell: TableCell) -> TableRow {
        self.0.cells.push(cell.take());
        self
    }
}
