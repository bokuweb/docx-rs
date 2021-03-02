use super::*;
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

    pub fn vertical_align(mut self, t: docx_rs::VAlignType) -> TableCell {
        self.0.property = self.0.property.vertical_align(t);
        self
    }

    pub fn grid_span(mut self, v: usize) -> TableCell {
        self.0.property = self.0.property.grid_span(v);
        self
    }

    pub fn width(mut self, v: usize) -> TableCell {
        self.0.property = self.0.property.width(v, docx_rs::WidthType::DXA);
        self
    }

    pub fn text_direction(mut self, t: docx_rs::TextDirectionType) -> TableCell {
        self.0.property = self.0.property.text_direction(t);
        self
    }

    pub fn set_border(mut self, border: TableCellBorder) -> TableCell {
        self.0.property = self.0.property.set_border(border.take());
        self
    }

    pub fn clear_border(mut self, position: docx_rs::BorderPosition) -> TableCell {
        self.0.property = self.0.property.clear_border(position);
        self
    }

    pub fn clear_all_border(mut self) -> TableCell {
        self.0.property = self.0.property.clear_all_border();
        self
    }
}
