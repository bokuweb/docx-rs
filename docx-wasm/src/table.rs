use super::*;
use docx_rs::WidthType;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Table(docx_rs::Table);

#[wasm_bindgen(js_name = createTable)]
pub fn create_table() -> Table {
    Table(docx_rs::Table::new(vec![]))
}

impl Table {
    pub fn take(self) -> docx_rs::Table {
        self.0
    }
}

#[wasm_bindgen]
impl Table {
    pub fn add_row(mut self, row: TableRow) -> Table {
        self.0.rows.push(docx_rs::TableChild::TableRow(row.take()));
        self
    }

    pub fn set_grid(mut self, grid: Vec<usize>) -> Table {
        self.0.grid = grid;
        self
    }

    pub fn style(mut self, style_id: &str) -> Table {
        self.0.property = self.0.property.style(style_id);
        self
    }

    pub fn indent(mut self, v: i32) -> Table {
        self.0 = self.0.indent(v);
        self
    }

    pub fn align(mut self, v: docx_rs::TableAlignmentType) -> Table {
        self.0 = self.0.align(v);
        self
    }

    pub fn width(mut self, w: usize) -> Table {
        self.0 = self.0.width(w, docx_rs::WidthType::Dxa);
        self
    }

    pub fn layout(mut self, t: docx_rs::TableLayoutType) -> Table {
        self.0 = self.0.layout(t);
        self
    }

    pub fn set_cell_margins(
        mut self,
        top: usize,
        right: usize,
        bottom: usize,
        left: usize,
    ) -> Table {
        let m = docx_rs::TableCellMargins::new().margin(top, right, bottom, left);
        self.0.property = self.0.property.set_margins(m);
        self
    }

    pub fn cell_margin_top(mut self, v: usize, t: WidthType) -> Table {
        self.0.property = self.0.property.cell_margin_top(v, t);
        self
    }

    pub fn cell_margin_right(mut self, v: usize, t: WidthType) -> Table {
        self.0.property = self.0.property.cell_margin_right(v, t);
        self
    }

    pub fn cell_margin_bottom(mut self, v: usize, t: WidthType) -> Table {
        self.0.property = self.0.property.cell_margin_bottom(v, t);
        self
    }

    pub fn cell_margin_left(mut self, v: usize, t: WidthType) -> Table {
        self.0.property = self.0.property.cell_margin_left(v, t);
        self
    }
}
