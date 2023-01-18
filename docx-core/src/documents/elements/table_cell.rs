use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;

use super::*;
use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;

#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TableCell {
    pub children: Vec<TableCellContent>,
    pub property: TableCellProperty,
    pub has_numbering: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TableCellContent {
    Paragraph(Paragraph),
    Table(Table),
    StructuredDataTag(Box<StructuredDataTag>),
    TableOfContents(Box<TableOfContents>),
}

impl Serialize for TableCellContent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            TableCellContent::Paragraph(ref s) => {
                let mut t = serializer.serialize_struct("Paragraph", 2)?;
                t.serialize_field("type", "paragraph")?;
                t.serialize_field("data", s)?;
                t.end()
            }
            TableCellContent::Table(ref s) => {
                let mut t = serializer.serialize_struct("Table", 2)?;
                t.serialize_field("type", "table")?;
                t.serialize_field("data", s)?;
                t.end()
            }
            TableCellContent::StructuredDataTag(ref r) => {
                let mut t = serializer.serialize_struct("StructuredDataTag", 2)?;
                t.serialize_field("type", "structuredDataTag")?;
                t.serialize_field("data", r)?;
                t.end()
            }
            TableCellContent::TableOfContents(ref r) => {
                let mut t = serializer.serialize_struct("TableOfContents", 2)?;
                t.serialize_field("type", "tableOfContents")?;
                t.serialize_field("data", r)?;
                t.end()
            }
        }
    }
}

impl TableCell {
    pub fn new() -> TableCell {
        Default::default()
    }

    pub fn add_paragraph(mut self, p: Paragraph) -> TableCell {
        if p.has_numbering {
            self.has_numbering = true
        }
        self.children.push(TableCellContent::Paragraph(p));
        self
    }

    pub fn add_table_of_contents(mut self, t: TableOfContents) -> Self {
        self.children
            .push(TableCellContent::TableOfContents(Box::new(t)));
        self
    }

    pub fn add_structured_data_tag(mut self, t: StructuredDataTag) -> Self {
        self.children
            .push(TableCellContent::StructuredDataTag(Box::new(t)));
        self
    }

    pub fn add_table(mut self, t: Table) -> TableCell {
        if t.has_numbering {
            self.has_numbering = true
        }
        self.children.push(TableCellContent::Table(t));
        self
    }

    pub fn vertical_merge(mut self, t: VMergeType) -> TableCell {
        self.property = self.property.vertical_merge(t);
        self
    }

    pub fn shading(mut self, s: Shading) -> TableCell {
        self.property = self.property.shading(s);
        self
    }

    pub fn vertical_align(mut self, t: VAlignType) -> TableCell {
        self.property = self.property.vertical_align(t);
        self
    }

    pub fn text_direction(mut self, t: TextDirectionType) -> TableCell {
        self.property = self.property.text_direction(t);
        self
    }

    pub fn grid_span(mut self, v: usize) -> TableCell {
        self.property = self.property.grid_span(v);
        self
    }

    pub fn width(mut self, v: usize, t: WidthType) -> TableCell {
        self.property = self.property.width(v, t);
        self
    }

    pub fn set_border(mut self, border: TableCellBorder) -> Self {
        self.property = self.property.set_border(border);
        self
    }

    pub fn set_borders(mut self, borders: TableCellBorders) -> Self {
        self.property = self.property.set_borders(borders);
        self
    }

    pub fn clear_border(mut self, position: TableCellBorderPosition) -> Self {
        self.property = self.property.clear_border(position);
        self
    }

    pub fn clear_all_border(mut self) -> Self {
        self.property = self.property.clear_all_border();
        self
    }
}

impl Default for TableCell {
    fn default() -> Self {
        let property = TableCellProperty::new();
        let children = vec![];
        Self {
            property,
            children,
            has_numbering: false,
        }
    }
}

impl BuildXML for TableCell {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        let mut b = b.open_table_cell().add_child(&self.property);
        for c in &self.children {
            match c {
                TableCellContent::Paragraph(p) => b = b.add_child(p),
                TableCellContent::Table(t) => {
                    b = b.add_child(t);
                    // INFO: We need to add empty paragraph when parent cell includes only cell.
                    if self.children.len() == 1 {
                        b = b.add_child(&Paragraph::new())
                    }
                }
                TableCellContent::StructuredDataTag(t) => b = b.add_child(t),
                TableCellContent::TableOfContents(t) => b = b.add_child(t),
            }
        }
        // INFO: We need to add empty paragraph when parent cell includes only cell.
        if self.children.is_empty() {
            b = b.add_child(&Paragraph::new())
        }
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
    fn test_cell() {
        let b = TableCell::new().build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:tc><w:tcPr /><w:p w14:paraId="12345678"><w:pPr><w:rPr /></w:pPr></w:p></w:tc>"#
        );
    }

    #[test]
    fn test_cell_add_p() {
        let b = TableCell::new()
            .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Hello")))
            .build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:tc><w:tcPr /><w:p w14:paraId="12345678"><w:pPr><w:rPr /></w:pPr><w:r><w:rPr /><w:t xml:space="preserve">Hello</w:t></w:r></w:p></w:tc>"#
        );
    }

    #[test]
    fn test_cell_json() {
        let c = TableCell::new()
            .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Hello")))
            .grid_span(2);
        assert_eq!(
            serde_json::to_string(&c).unwrap(),
            r#"{"children":[{"type":"paragraph","data":{"id":"12345678","children":[{"type":"run","data":{"runProperty":{},"children":[{"type":"text","data":{"preserveSpace":true,"text":"Hello"}}]}}],"property":{"runProperty":{},"tabs":[]},"hasNumbering":false}}],"property":{"width":null,"borders":null,"gridSpan":2,"verticalMerge":null,"verticalAlign":null,"textDirection":null,"shading":null},"hasNumbering":false}"#,
        );
    }
}
