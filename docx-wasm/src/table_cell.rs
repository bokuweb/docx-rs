use super::*;
use docx;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct TableCell(docx::TableCell);

#[wasm_bindgen(js_name = createTableCell)]
pub fn create_table_cell() -> TableCell {
    TableCell(docx::TableCell::new())
}

impl TableCell {
    pub fn take(self) -> docx::TableCell {
        self.0
    }
}

#[wasm_bindgen]
impl TableCell {
    pub fn add_paragraph(mut self, p: Paragraph) -> TableCell {
        self.0
            .contents
            .push(docx::TableCellContent::Paragraph(p.take()));
        self
    }

    pub fn vertical_merge(mut self, t: docx::VMergeType) -> TableCell {
        self.0.property = self.0.property.vertical_merge(t);
        self
    }

    pub fn grid_span(mut self, v: usize) -> TableCell {
        self.0.property = self.0.property.grid_span(v);
        self
    }

    pub fn width(mut self, v: usize) -> TableCell {
        self.0.property = self.0.property.width(v, docx::WidthType::DXA);
        self
    }
}
