use super::{TableCell, TableRowProperty};
use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone)]
pub struct TableRow {
    property: TableRowProperty,
    cells: Vec<TableCell>,
}

impl TableRow {
    pub fn new(cells: Vec<TableCell>) -> TableRow {
        let property = TableRowProperty::new();
        Self { property, cells }
    }
}

impl BuildXML for TableRow {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new()
            .open_table_row()
            .add_child(&self.property)
            .add_children(&self.cells);
        b.close().build()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_row() {
        let b = TableRow::new(vec![TableCell::new()]).build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:tr><w:trPr /><w:tc><w:tcPr><w:tcBorders><w:top w:val="single" w:sz="2" w:space="0" w:color="000000" /><w:left w:val="single" w:sz="2" w:space="0" w:color="000000" /><w:bottom w:val="single" w:sz="2" w:space="0" w:color="000000" /><w:insideH w:val="single" w:sz="2" w:space="0" w:color="000000" /></w:tcBorders></w:tcPr></w:tc></w:tr>"#
        );
    }
}
