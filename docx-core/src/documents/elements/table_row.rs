use serde::Serialize;

use super::{TableCell, TableRowProperty};
use crate::xml_builder::*;
use crate::{documents::BuildXML, HeightRule};

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

    pub fn grid_after(mut self, grid_after: u32) -> TableRow {
        self.property = self.property.grid_after(grid_after);
        self
    }

    pub fn width_after(mut self, w: f32) -> TableRow {
        self.property = self.property.width_after(w);
        self
    }

    pub fn row_height(mut self, h: f32) -> TableRow {
        self.property = self.property.row_height(h);
        self
    }

    pub fn height_rule(mut self, r: HeightRule) -> TableRow {
        self.property = self.property.height_rule(r);
        self
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
            r#"<w:tr><w:trPr /><w:tc><w:tcPr /><w:p w14:paraId="12345678"><w:pPr><w:rPr /></w:pPr></w:p></w:tc></w:tr>"#
        );
    }

    #[test]
    fn test_row_json() {
        let r = TableRow::new(vec![TableCell::new()]);
        assert_eq!(
            serde_json::to_string(&r).unwrap(),
            r#"{"cells":[{"children":[],"property":{"width":null,"borders":null,"gridSpan":null,"verticalMerge":null,"verticalAlign":null,"textDirection":null},"hasNumbering":false}],"hasNumbering":false,"property":{"gridAfter":null,"widthAfter":null,"rowHeight":null,"heightRule":null}}"#
        );
    }
}
