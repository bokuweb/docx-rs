use docx_rs;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct TableCellBorder(docx_rs::TableCellBorder);

#[wasm_bindgen(js_name = createTableCellBorder)]
pub fn create_table_cell_border(position: docx_rs::BorderPosition) -> TableCellBorder {
    TableCellBorder(docx_rs::TableCellBorder::new(position))
}

impl TableCellBorder {
    pub fn take(self) -> docx_rs::TableCellBorder {
        self.0
    }
}

#[wasm_bindgen]
impl TableCellBorder {
    pub fn color(mut self, color: String) -> TableCellBorder {
        self.0.color = color;
        self
    }

    pub fn border_type(mut self, border_type: docx_rs::BorderType) -> TableCellBorder {
        self.0.border_type = border_type;
        self
    }
}
