use super::*;
use docx_rs::CharacterSpacingValues;
use wasm_bindgen::prelude::*;

extern crate console_error_panic_hook;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Docx(docx_rs::Docx);

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn createDocx() -> Docx {
    use std::panic;
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    Docx(docx_rs::Docx::new())
}

#[wasm_bindgen]
impl Docx {
    pub fn add_paragraph(mut self, p: Paragraph) -> Self {
        self.0 = self.0.add_paragraph(p.take());
        self
    }

    pub fn add_table_of_contents(mut self, t: TableOfContents) -> Self {
        self.0 = self.0.add_table_of_contents(t.take());
        self
    }

    pub fn add_bookmark_start(mut self, id: usize, name: &str) -> Self {
        self.0 = self.0.add_bookmark_start(id, name);
        self
    }

    pub fn add_bookmark_end(mut self, id: usize) -> Docx {
        self.0 = self.0.add_bookmark_end(id);
        self
    }

    pub fn add_table(mut self, t: Table) -> Docx {
        self.0.document = self.0.document.add_table(t.take());
        self
    }

    pub fn add_abstract_numbering(mut self, num: AbstractNumbering) -> Docx {
        self.0.numberings = self.0.numberings.add_abstract_numbering(num.take());
        self
    }

    pub fn add_numbering(mut self, num: Numbering) -> Docx {
        self.0.numberings = self.0.numberings.add_numbering(num.take());
        self
    }

    pub fn created_at(mut self, date: &str) -> Self {
        self.0.doc_props = self.0.doc_props.created_at(date);
        self
    }

    pub fn updated_at(mut self, date: &str) -> Self {
        self.0.doc_props = self.0.doc_props.updated_at(date);
        self
    }

    pub fn custom_property(mut self, name: &str, item: &str) -> Self {
        self.0.doc_props = self.0.doc_props.custom_property(name, item);
        self
    }

    pub fn doc_id(mut self, id: &str) -> Docx {
        self.0 = self.0.doc_id(id);
        self
    }

    pub fn add_doc_var(mut self, name: &str, val: &str) -> Docx {
        self.0 = self.0.add_doc_var(name, val);
        self
    }

    pub fn default_tab_stop(mut self, stop: usize) -> Docx {
        self.0 = self.0.default_tab_stop(stop);
        self
    }

    pub fn set_adjust_line_height_in_table(mut self) -> Self {
        self.0.settings = self.0.settings.adjust_line_height_in_table();
        self
    }

    pub fn character_spacing_control(mut self, v: CharacterSpacingValues) -> Self {
        self.0.settings = self.0.settings.character_spacing_control(v);
        self
    }

    pub fn header(mut self, header: Header) -> Self {
        self.0 = self.0.header(header.take());
        self
    }

    pub fn first_header(mut self, header: Header) -> Self {
        self.0 = self.0.first_header(header.take());
        self
    }

    pub fn even_header(mut self, header: Header) -> Self {
        self.0 = self.0.even_header(header.take());
        self
    }

    pub fn footer(mut self, footer: Footer) -> Self {
        self.0 = self.0.footer(footer.take());
        self
    }

    pub fn first_footer(mut self, footer: Footer) -> Self {
        self.0 = self.0.first_footer(footer.take());
        self
    }

    pub fn even_footer(mut self, footer: Footer) -> Self {
        self.0 = self.0.even_footer(footer.take());
        self
    }

    pub fn page_size(mut self, w: u32, h: u32) -> Docx {
        self.0 = self.0.page_size(w, h);
        self
    }

    pub fn page_orient(mut self, o: docx_rs::PageOrientationType) -> Docx {
        self.0 = self.0.page_orient(o);
        self
    }

    pub fn page_margin(mut self, margin: PageMargin) -> Docx {
        self.0 = self.0.page_margin(margin.take());
        self
    }

    pub fn add_style(mut self, s: Style) -> Self {
        self.0.styles = self.0.styles.add_style(s.take());
        self
    }

    pub fn default_size(mut self, size: usize) -> Self {
        self.0.styles = self.0.styles.default_size(size);
        self
    }

    pub fn default_spacing(mut self, spacing: i32) -> Self {
        self.0.styles = self.0.styles.default_spacing(spacing);
        self
    }

    pub fn default_fonts(mut self, font: RunFonts) -> Self {
        self.0.styles = self.0.styles.default_fonts(font.take());
        self
    }

    pub fn taskpanes(mut self) -> Self {
        self.0 = self.0.taskpanes();
        self
    }

    pub fn web_extension(mut self, ext: WebExtension) -> Self {
        self.0 = self.0.web_extension(ext.take());
        self
    }

    pub fn add_custom_item(mut self, id: &str, xml: &str) -> Self {
        self.0 = self.0.add_custom_item(id, xml);
        self
    }

    pub fn doc_grid(
        mut self,
        grid_type: docx_rs::DocGridType,
        line_pitch: Option<usize>,
        char_space: Option<isize>,
    ) -> Self {
        let mut doc_grid = docx_rs::DocGrid::with_empty().grid_type(grid_type);
        if let Some(line_pitch) = line_pitch {
            doc_grid = doc_grid.line_pitch(line_pitch);
        }
        if let Some(char_space) = char_space {
            doc_grid = doc_grid.char_space(char_space);
        }
        self.0.document = self.0.document.doc_grid(doc_grid);
        self
    }

    pub fn build(mut self, has_numberings: bool) -> Result<Vec<u8>, JsValue> {
        let buf = Vec::new();
        let mut cur = std::io::Cursor::new(buf);
        if has_numberings {
            self.0.document_rels.has_numberings = true;
        }
        let res = self.0.build().pack(&mut cur);
        if res.is_err() {
            return Err(format!("{:?}", res).into());
        }
        Ok(cur.into_inner())
    }

    pub fn json_with_update_comments(&mut self) -> String {
        self.0.json_with_update_comments()
    }
}
