use super::{TableCell, TableRowProperty};
use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone)]
pub struct TableRow<'a> {
    pub(crate) cells: Vec<TableCell<'a>>,
    property: TableRowProperty,
}

impl<'a> TableRow<'a> {
    pub fn new(cells: Vec<TableCell<'a>>) -> TableRow<'a> {
        let property = TableRowProperty::new();
        Self { property, cells }
    }
}

impl<'a> BuildXML for TableRow<'a> {
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
            r#"<w:tr><w:trPr /><w:tc><w:tcPr /></w:tc></w:tr>"#
        );
    }
}
