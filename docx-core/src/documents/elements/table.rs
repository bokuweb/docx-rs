use serde::Serialize;

use super::{TableGrid, TableProperty, TableRow};
use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Table {
    pub rows: Vec<TableRow>,
    pub grid: Vec<usize>,
    pub has_numbering: bool,
    pub property: TableProperty,
}

impl Table {
    pub fn new(rows: Vec<TableRow>) -> Table {
        let property = TableProperty::new();
        let has_numbering = rows.iter().any(|c| c.has_numbering);
        let grid = vec![];
        Self {
            property,
            rows,
            grid,
            has_numbering,
        }
    }

    pub fn add_row(mut self, row: TableRow) -> Table {
        self.rows.push(row);
        self
    }

    pub fn set_grid(mut self, grid: Vec<usize>) -> Table {
        self.grid = grid;
        self
    }

    pub fn indent(mut self, v: usize) -> Table {
        self.property = self.property.indent(v);
        self
    }

    pub fn align(mut self, v: TableAlignmentType) -> Table {
        self.property = self.property.align(v);
        self
    }

    pub fn width(mut self, w: usize, t: WidthType) -> Table {
        self.property = self.property.width(w, t);
        self
    }
}

impl BuildXML for Table {
    fn build(&self) -> Vec<u8> {
        let grid = TableGrid::new(self.grid.clone());
        let b = XMLBuilder::new()
            .open_table()
            .add_child(&self.property)
            .add_child(&grid)
            .add_children(&self.rows);
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
    fn test_table() {
        let b = Table::new(vec![TableRow::new(vec![])]).build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:tbl><w:tblPr><w:tblW w:w="0" w:type="dxa" /><w:jc w:val="left" /><w:tblBorders><w:top w:val="single" w:sz="2" w:space="0" w:color="000000" /><w:left w:val="single" w:sz="2" w:space="0" w:color="000000" /><w:bottom w:val="single" w:sz="2" w:space="0" w:color="000000" /><w:right w:val="single" w:sz="2" w:space="0" w:color="000000" /><w:insideH w:val="single" w:sz="2" w:space="0" w:color="000000" /><w:insideV w:val="single" w:sz="2" w:space="0" w:color="000000" /></w:tblBorders><w:tblCellMar>
  <w:top w:w="55" w:type="dxa" />
  <w:left w:w="54" w:type="dxa" />
  <w:bottom w:w="55" w:type="dxa" />
  <w:right w:w="55" w:type="dxa" />
</w:tblCellMar></w:tblPr><w:tblGrid /><w:tr><w:trPr /></w:tr></w:tbl>"#
        );
    }

    #[test]
    fn test_table_grid() {
        let b = Table::new(vec![TableRow::new(vec![])])
            .set_grid(vec![100, 200])
            .build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:tbl><w:tblPr><w:tblW w:w="0" w:type="dxa" /><w:jc w:val="left" /><w:tblBorders><w:top w:val="single" w:sz="2" w:space="0" w:color="000000" /><w:left w:val="single" w:sz="2" w:space="0" w:color="000000" /><w:bottom w:val="single" w:sz="2" w:space="0" w:color="000000" /><w:right w:val="single" w:sz="2" w:space="0" w:color="000000" /><w:insideH w:val="single" w:sz="2" w:space="0" w:color="000000" /><w:insideV w:val="single" w:sz="2" w:space="0" w:color="000000" /></w:tblBorders><w:tblCellMar>
  <w:top w:w="55" w:type="dxa" />
  <w:left w:w="54" w:type="dxa" />
  <w:bottom w:w="55" w:type="dxa" />
  <w:right w:w="55" w:type="dxa" />
</w:tblCellMar></w:tblPr><w:tblGrid>
  <w:gridCol w:w="100" w:type="dxa" />
  <w:gridCol w:w="200" w:type="dxa" />
</w:tblGrid><w:tr><w:trPr /></w:tr></w:tbl>"#
        );
    }

    #[test]
    fn test_table_json() {
        let t = Table::new(vec![]).set_grid(vec![100, 200, 300]);
        assert_eq!(
            serde_json::to_string(&t).unwrap(),
            r#"{"rows":[],"grid":[100,200,300],"hasNumbering":false,"property":{"width":{"width":0,"widthType":"Auto"},"justification":"left","borders":{"top":{"position":"top","borderType":"single","size":2,"space":0,"color":"000000"},"left":{"position":"left","borderType":"single","size":2,"space":0,"color":"000000"},"bottom":{"position":"bottom","borderType":"single","size":2,"space":0,"color":"000000"},"right":{"position":"right","borderType":"single","size":2,"space":0,"color":"000000"},"insideH":{"position":"insideH","borderType":"single","size":2,"space":0,"color":"000000"},"insideV":{"position":"insideV","borderType":"single","size":2,"space":0,"color":"000000"}},"margins":{"top":55,"left":54,"bottom":55,"right":55},"indent":null}}"#
        );
    }
}
