use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;

#[derive(Debug, Clone)]
pub struct TableCellWidth {
    width: usize,
    width_type: WidthType,
}

impl TableCellWidth {
    pub fn new(width: usize, width_type: WidthType) -> TableCellWidth {
        TableCellWidth { width, width_type }
    }
}

impl BuildXML for TableCellWidth {
    fn build(&self) -> Vec<u8> {
        XMLBuilder::new()
            .table_cell_width(self.width, WidthType::DXA)
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
    fn test_table_width() {
        let b = TableCellWidth::new(20, WidthType::DXA).build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:tcW w:w="20" w:type="dxa" />"#
        );
    }
}
