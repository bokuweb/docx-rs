use super::{TableGrid, TableProperty, TableRow};
use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;

#[derive(Debug, Clone)]
pub struct Table {
    pub rows: Vec<TableRow>,
    pub grid: Vec<usize>,
    property: TableProperty,
}

impl Table {
    pub fn new(rows: Vec<TableRow>) -> Table {
        let property = TableProperty::new();
        let grid = vec![];
        Self {
            property,
            rows,
            grid,
        }
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
}
