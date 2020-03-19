use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;

#[derive(Debug, Clone)]
pub struct TableGrid {
    grid: Vec<usize>,
}

impl TableGrid {
    pub fn new(grid: Vec<usize>) -> TableGrid {
        TableGrid { grid }
    }
}

impl BuildXML for TableGrid {
    fn build(&self) -> Vec<u8> {
        let mut base = XMLBuilder::new().open_table_grid();
        for g in &self.grid {
            base = base.grid_column(*g as i32, WidthType::DXA);
        }
        base.close().build()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_table_indent() {
        let b = TableGrid::new(vec![100, 200]).build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:tblGrid>
  <w:gridCol w:w="100" w:type="dxa" />
  <w:gridCol w:w="200" w:type="dxa" />
</w:tblGrid>"#
        );
    }
}
