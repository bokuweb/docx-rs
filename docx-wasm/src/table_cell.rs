use super::*;
use docx_rs;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct TableCell(docx_rs::TableCell);

#[wasm_bindgen(js_name = createTableCell)]
pub fn create_table_cell() -> TableCell {
    TableCell(docx_rs::TableCell::new())
}

impl TableCell {
    pub fn take(self) -> docx_rs::TableCell {
        self.0
    }
}

#[wasm_bindgen]
impl TableCell {
    pub fn add_paragraph(mut self, p: Paragraph) -> TableCell {
        self.0
            .children
            .push(docx_rs::TableCellContent::Paragraph(p.take()));
        self
    }

    pub fn vertical_merge(mut self, t: docx_rs::VMergeType) -> TableCell {
        self.0.property = self.0.property.vertical_merge(t);
        self
    }

    pub fn grid_span(mut self, v: usize) -> TableCell {
        self.0.property = self.0.property.grid_span(v);
        self
    }

    pub fn width(mut self, v: usize) -> TableCell {
        self.0.property = self.0.property.width(v, docx_rs::WidthType::Dxa);
        self
    }
}
