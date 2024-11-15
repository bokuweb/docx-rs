use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;
use std::io::Write;

use super::{Delete, Insert, TableCell, TableRowProperty};
use crate::xml_builder::*;
use crate::{documents::BuildXML, HeightRule};

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TableRow {
    // 2. w:trPr
    pub property: TableRowProperty,
    // 3. other elements
    pub cells: Vec<TableRowChild>,
    pub has_numbering: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TableRowChild {
    TableCell(TableCell),
}

impl BuildXML for TableRowChild {
    fn build_to<W: Write>(
        &self,
        stream: xml::writer::EventWriter<W>,
    ) -> xml::writer::Result<xml::writer::EventWriter<W>> {
        match self {
            TableRowChild::TableCell(v) => v.build_to(stream),
        }
    }
}

impl TableRow {
    pub fn new(cells: Vec<TableCell>) -> TableRow {
        let property = TableRowProperty::new();
        let has_numbering = cells.iter().any(|c| c.has_numbering);
        let cells = cells.into_iter().map(TableRowChild::TableCell).collect();
        Self {
            cells,
            property,
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

    pub fn grid_before(mut self, grid_before: u32) -> TableRow {
        self.property = self.property.grid_before(grid_before);
        self
    }

    pub fn width_before(mut self, w: f32) -> TableRow {
        self.property = self.property.width_before(w);
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

    pub fn delete(mut self, d: Delete) -> TableRow {
        self.property = self.property.delete(d);
        self
    }

    pub fn insert(mut self, i: Insert) -> TableRow {
        self.property = self.property.insert(i);
        self
    }

    pub fn cant_split(mut self) -> TableRow {
        self.property = self.property.cant_split();
        self
    }
}

impl BuildXML for TableRow {
    fn build_to<W: Write>(
        &self,
        stream: xml::writer::EventWriter<W>,
    ) -> xml::writer::Result<xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_table_row()?
            .add_child(&self.property)?
            .add_children(&self.cells)?
            .close()?
            .into_inner()
    }
}

impl Serialize for TableRowChild {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            TableRowChild::TableCell(ref r) => {
                let mut t = serializer.serialize_struct("TableCell", 2)?;
                t.serialize_field("type", "tableCell")?;
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
    fn test_row() {
        let b = TableRow::new(vec![TableCell::new()]).build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:tr><w:trPr /><w:tc><w:tcPr /><w:p w14:paraId="12345678"><w:pPr /></w:p></w:tc></w:tr>"#
        );
    }

    #[test]
    fn test_row_json() {
        let r = TableRow::new(vec![TableCell::new()]);
        assert_eq!(
            serde_json::to_string(&r).unwrap(),
            r#"{"property":{"gridBefore":null,"gridAfter":null,"widthBefore":null,"widthAfter":null},"cells":[{"type":"tableCell","data":{"children":[],"property":{"width":null,"gridSpan":null,"verticalMerge":null,"borders":null,"shading":null,"textDirection":null,"verticalAlign":null},"hasNumbering":false}}],"hasNumbering":false}"#
        );
    }

    #[test]
    fn test_row_cant_split() {
        let b = TableRow::new(vec![TableCell::new()]).cant_split().build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:tr><w:trPr><w:cantSplit /></w:trPr><w:tc><w:tcPr /><w:p w14:paraId="12345678"><w:pPr /></w:p></w:tc></w:tr>"#
        );
    }
}
