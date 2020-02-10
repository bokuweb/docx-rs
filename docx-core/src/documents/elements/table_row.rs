use serde::Serialize;

use super::{TableCell, TableRowProperty};
use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TableRow {
    pub cells: Vec<TableCell>,
    pub has_numbering: bool,
    pub property: TableRowProperty,
}

impl TableRow {
    pub fn new(cells: Vec<TableCell>) -> TableRow {
        let property = TableRowProperty::new();
        let has_numbering = cells.iter().any(|c| c.has_numbering);
        Self {
            property,
            cells,
            has_numbering,
        }
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
            r#"<w:tr><w:trPr /><w:tc><w:tcPr /></w:tc></w:tr>"#
        );
    }

    #[test]
    fn test_row_json() {
        let r = TableRow::new(vec![TableCell::new()]);
        assert_eq!(
            serde_json::to_string(&r).unwrap(),
            r#"{"cells":[{"children":[],"property":{"width":null,"borders":null,"gridSpan":null,"verticalMerge":null},"hasNumbering":false}],"hasNumbering":false,"property":{}}"#
        );
    }
}
