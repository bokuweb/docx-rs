use super::*;
use docx_core;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Table(docx_core::Table);

#[wasm_bindgen(js_name = createTable)]
pub fn create_table() -> Table {
    Table(docx_core::Table::new(vec![]))
}

impl Table {
    pub fn take(self) -> docx_core::Table {
        self.0
    }
}

#[wasm_bindgen]
impl Table {
    pub fn add_row(mut self, row: TableRow) -> Table {
        self.0.rows.push(row.take());
        self
    }

    pub fn set_grid(mut self, grid: Vec<usize>) -> Table {
        self.0.grid = grid;
        self
    }
}
