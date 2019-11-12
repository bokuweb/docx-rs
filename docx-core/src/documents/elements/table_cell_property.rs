use super::{TableCellBorders, TableCellWidth};
use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;

#[derive(Debug, Clone)]
pub struct TableCellProperty {
    width: TableCellWidth,
    borders: TableCellBorders,
}

impl TableCellProperty {
    pub fn new(w: usize) -> TableCellProperty {
        TableCellProperty {
            width: TableCellWidth::new(w, WidthType::DXA),
            borders: TableCellBorders::new(),
        }
    }

    pub fn width(mut self, v: usize) -> TableCellProperty {
        self.width = TableCellWidth::new(v, WidthType::DXA);
        self
    }
}

impl BuildXML for TableCellProperty {
    fn build(&self) -> Vec<u8> {
        XMLBuilder::new()
            .open_table_cell_property()
            .add_child(&self.width)
            .add_child(&self.borders)
            .close()
            .build()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_default() {
        let c = TableCellProperty::new(200);
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:tcPr><w:tcW w:w="200" w:type="dxa" /><w:tcBorders><w:top w:val="single" w:sz="2" w:space="0" w:color="000000" /><w:left w:val="single" w:sz="2" w:space="0" w:color="000000" /><w:bottom w:val="single" w:sz="2" w:space="0" w:color="000000" /><w:insideH w:val="single" w:sz="2" w:space="0" w:color="000000" /></w:tcBorders></w:tcPr>"#
        );
    }
}
