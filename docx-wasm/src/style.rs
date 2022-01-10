use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Style(docx_rs::Style);

#[wasm_bindgen(js_name = createStyle)]
pub fn create_style(style_id: &str, style_type: docx_rs::StyleType) -> Style {
    Style(docx_rs::Style::new(style_id, style_type))
}

#[wasm_bindgen]
impl Style {
    pub fn name(mut self, name: &str) -> Self {
        self.0.name = docx_rs::Name::new(name);
        self
    }

    pub fn based_on(mut self, base: &str) -> Self {
        self.0.based_on = Some(docx_rs::BasedOn::new(base));
        self
    }

    pub fn size(mut self, size: usize) -> Self {
        self.0.run_property = self.0.run_property.size(size);
        self
    }

    pub fn color(mut self, color: &str) -> Self {
        self.0.run_property = self.0.run_property.color(color);
        self
    }

    pub fn highlight(mut self, color: &str) -> Self {
        self.0.run_property = self.0.run_property.highlight(color);
        self
    }

    pub fn bold(mut self) -> Self {
        self.0.run_property = self.0.run_property.bold();
        self
    }

    pub fn italic(mut self) -> Self {
        self.0.run_property = self.0.run_property.italic();
        self
    }

    pub fn underline(mut self, line_type: &str) -> Self {
        self.0.run_property = self.0.run_property.underline(line_type);
        self
    }

    pub fn vanish(mut self) -> Self {
        self.0.run_property = self.0.run_property.vanish();
        self
    }

    pub fn align(mut self, alignment_type: docx_rs::AlignmentType) -> Self {
        self.0.paragraph_property = self.0.paragraph_property.align(alignment_type);
        self
    }

    pub fn indent(
        mut self,
        left: i32,
        special_indent_kind: Option<docx_rs::SpecialIndentKind>,
        special_indent_size: Option<i32>,
    ) -> Self {
        let special_indent = create_special_indent(special_indent_kind, special_indent_size);
        self.0.paragraph_property =
            self.0
                .paragraph_property
                .indent(Some(left), special_indent, None, None);
        self
    }

    pub fn outline_lvl(mut self, l: usize) -> Self {
        self.0.paragraph_property = self.0.paragraph_property.outline_lvl(l);
        self
    }

    // pub fn run_property(mut self, p: docx_rs::RunProperty) -> Self {
    //     self.0.run_property = p;
    //     self
    // }

    // pub fn paragraph_property(mut self, p: docx_rs::ParagraphProperty) -> Self {
    //     self.0.paragraph_property = p;
    //     self
    // }

    pub fn table_property(mut self, p: docx_rs::TableProperty) -> Self {
        self.0.table_property = p;
        self
    }

    pub fn table_cell_property(mut self, p: docx_rs::TableCellProperty) -> Self {
        self.0.table_cell_property = p;
        self
    }
}

impl Style {
    pub fn take(self) -> docx_rs::Style {
        self.0
    }
}
