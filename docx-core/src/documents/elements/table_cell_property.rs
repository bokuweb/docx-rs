use super::{TableCellBorders, TableCellWidth};
use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;

#[derive(Debug, Clone)]
pub struct TableCellProperty {
    width: Option<TableCellWidth>,
    borders: Option<TableCellBorders>,
}

impl TableCellProperty {
    pub fn new() -> TableCellProperty {
        TableCellProperty {
            width: None,
            borders: None,
        }
    }

    pub fn width(mut self, v: usize, t: WidthType) -> TableCellProperty {
        self.width = Some(TableCellWidth::new(v, t));
        self
    }
}

impl BuildXML for TableCellProperty {
    fn build(&self) -> Vec<u8> {
        XMLBuilder::new()
            .open_table_cell_property()
            .add_optional_child(&self.width)
            .add_optional_child(&self.borders)
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
        let c = TableCellProperty::new();
        let b = c.build();
        assert_eq!(str::from_utf8(&b).unwrap(), r#"<w:tcPr />"#);
    }
}
