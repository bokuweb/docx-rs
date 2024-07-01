use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;

use super::*;
use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Table {
    pub rows: Vec<TableChild>,
    pub grid: Vec<usize>,
    pub has_numbering: bool,
    pub property: TableProperty,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TableChild {
    TableRow(TableRow),
}

impl BuildXML for TableChild {
    fn build(&self) -> Vec<u8> {
        match self {
            TableChild::TableRow(v) => v.build(),
        }
    }
}

impl Table {
    pub fn new(rows: Vec<TableRow>) -> Table {
        let property = TableProperty::new();
        let has_numbering = rows.iter().any(|c| c.has_numbering);
        let grid = vec![];
        let rows = rows.into_iter().map(TableChild::TableRow).collect();
        Self {
            property,
            rows,
            grid,
            has_numbering,
        }
    }

    pub fn without_borders(rows: Vec<TableRow>) -> Table {
        let property = TableProperty::without_borders();
        let has_numbering = rows.iter().any(|c| c.has_numbering);
        let grid = vec![];
        let rows = rows.into_iter().map(TableChild::TableRow).collect();
        Self {
            property,
            rows,
            grid,
            has_numbering,
        }
    }

    pub fn add_row(mut self, row: TableRow) -> Table {
        self.rows.push(TableChild::TableRow(row));
        self
    }

    pub fn set_grid(mut self, grid: Vec<usize>) -> Table {
        self.grid = grid;
        self
    }

    pub fn indent(mut self, v: i32) -> Table {
        self.property = self.property.indent(v);
        self
    }

    pub fn align(mut self, v: TableAlignmentType) -> Table {
        self.property = self.property.align(v);
        self
    }

    pub fn style(mut self, s: impl Into<String>) -> Table {
        self.property = self.property.style(s);
        self
    }

    pub fn layout(mut self, t: TableLayoutType) -> Table {
        self.property = self.property.layout(t);
        self
    }

    pub fn position(mut self, p: TablePositionProperty) -> Self {
        self.property = self.property.position(p);
        self
    }

    pub fn width(mut self, w: usize, t: WidthType) -> Table {
        self.property = self.property.width(w, t);
        self
    }

    pub fn margins(mut self, margins: TableCellMargins) -> Self {
        self.property = self.property.set_margins(margins);
        self
    }

    pub fn set_borders(mut self, borders: TableBorders) -> Self {
        self.property = self.property.set_borders(borders);
        self
    }

    pub fn set_border(mut self, border: TableBorder) -> Self {
        self.property = self.property.set_border(border);
        self
    }

    pub fn clear_border(mut self, position: TableBorderPosition) -> Self {
        self.property = self.property.clear_border(position);
        self
    }

    pub fn clear_all_border(mut self) -> Self {
        self.property = self.property.clear_all_border();
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

impl BuildXML for Box<Table> {
    fn build(&self) -> Vec<u8> {
        Table::build(self)
    }
}

impl Serialize for TableChild {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            TableChild::TableRow(ref r) => {
                let mut t = serializer.serialize_struct("TableRow", 2)?;
                t.serialize_field("type", "tableRow")?;
                t.serialize_field("data", r)?;
                t.end()
            }
        }
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
            r#"<w:tbl><w:tblPr><w:tblW w:w="0" w:type="auto" /><w:jc w:val="left" /><w:tblBorders><w:top w:val="single" w:sz="2" w:space="0" w:color="000000" /><w:left w:val="single" w:sz="2" w:space="0" w:color="000000" /><w:bottom w:val="single" w:sz="2" w:space="0" w:color="000000" /><w:right w:val="single" w:sz="2" w:space="0" w:color="000000" /><w:insideH w:val="single" w:sz="2" w:space="0" w:color="000000" /><w:insideV w:val="single" w:sz="2" w:space="0" w:color="000000" /></w:tblBorders></w:tblPr><w:tblGrid /><w:tr><w:trPr /></w:tr></w:tbl>"#
        );
    }

    #[test]
    fn test_table_grid() {
        let b = Table::new(vec![TableRow::new(vec![])])
            .set_grid(vec![100, 200])
            .build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:tbl><w:tblPr><w:tblW w:w="0" w:type="auto" /><w:jc w:val="left" /><w:tblBorders><w:top w:val="single" w:sz="2" w:space="0" w:color="000000" /><w:left w:val="single" w:sz="2" w:space="0" w:color="000000" /><w:bottom w:val="single" w:sz="2" w:space="0" w:color="000000" /><w:right w:val="single" w:sz="2" w:space="0" w:color="000000" /><w:insideH w:val="single" w:sz="2" w:space="0" w:color="000000" /><w:insideV w:val="single" w:sz="2" w:space="0" w:color="000000" /></w:tblBorders></w:tblPr><w:tblGrid>
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
            r#"{"rows":[],"grid":[100,200,300],"hasNumbering":false,"property":{"width":{"width":0,"widthType":"auto"},"justification":"left","borders":{"top":{"borderType":"single","size":2,"color":"000000","position":"top","space":0},"left":{"borderType":"single","size":2,"color":"000000","position":"left","space":0},"bottom":{"borderType":"single","size":2,"color":"000000","position":"bottom","space":0},"right":{"borderType":"single","size":2,"color":"000000","position":"right","space":0},"insideH":{"borderType":"single","size":2,"color":"000000","position":"insideH","space":0},"insideV":{"borderType":"single","size":2,"color":"000000","position":"insideV","space":0}}}}"#
        );
    }
}
