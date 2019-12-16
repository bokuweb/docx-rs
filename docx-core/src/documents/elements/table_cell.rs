use super::{Paragraph, TableCellProperty};
use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;

#[derive(Debug, Clone)]
pub struct TableCell {
    pub contents: Vec<TableCellContent>,
    pub property: TableCellProperty,
    pub has_numbering: bool,
}

#[derive(Debug, Clone)]
pub enum TableCellContent {
    Paragraph(Paragraph),
}

impl TableCell {
    pub fn new() -> TableCell {
        let property = TableCellProperty::new();
        let contents = vec![];
        Self {
            property,
            contents,
            has_numbering: false,
        }
    }

    pub fn add_paragraph(mut self, p: Paragraph) -> TableCell {
        if p.has_numbering {
            self.has_numbering = true
        }
        self.contents.push(TableCellContent::Paragraph(p));
        self
    }

    pub fn vertical_merge(mut self, t: VMergeType) -> TableCell {
        self.property = self.property.vertical_merge(t);
        self
    }

    pub fn grid_span(mut self, v: usize) -> TableCell {
        self.property = self.property.grid_span(v);
        self
    }
}

impl BuildXML for TableCell {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        let mut b = b.open_table_cell().add_child(&self.property);
        for c in &self.contents {
            match c {
                TableCellContent::Paragraph(p) => b = b.add_child(p),
            }
        }
        b.close().build()
    }
}

#[cfg(test)]
mod tests {

    use super::super::*;
    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_cell() {
        let b = TableCell::new().build();
        assert_eq!(str::from_utf8(&b).unwrap(), r#"<w:tc><w:tcPr /></w:tc>"#);
    }

    #[test]
    fn test_cell_add_p() {
        let b = TableCell::new()
            .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Hello")))
            .build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:tc><w:tcPr /><w:p><w:pPr><w:pStyle w:val="Normal" /><w:rPr /></w:pPr><w:r><w:rPr /><w:t xml:space="preserve">Hello</w:t></w:r></w:p></w:tc>"#
        );
    }
}
