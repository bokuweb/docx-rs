use serde::Serialize;

use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TableCellMargins {
    top: usize,
    left: usize,
    bottom: usize,
    right: usize,
}

impl Default for TableCellMargins {
    fn default() -> TableCellMargins {
        TableCellMargins {
            top: 55,
            left: 54,
            bottom: 55,
            right: 55,
        }
    }
}

impl TableCellMargins {
    pub fn new() -> TableCellMargins {
        Default::default()
    }

    pub fn margin(self, top: usize, left: usize, bottom: usize, right: usize) -> TableCellMargins {
        TableCellMargins {
            top,
            left,
            bottom,
            right,
        }
    }
}

impl BuildXML for TableCellMargins {
    fn build(&self) -> Vec<u8> {
        XMLBuilder::new()
            .open_table_cell_margins()
            .margin_top(self.top, WidthType::Dxa)
            .margin_left(self.left, WidthType::Dxa)
            .margin_bottom(self.bottom, WidthType::Dxa)
            .margin_right(self.right, WidthType::Dxa)
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
    fn test_table_cell_margin() {
        let b = TableCellMargins::new().build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:tblCellMar>
  <w:top w:w="55" w:type="Dxa" />
  <w:left w:w="54" w:type="Dxa" />
  <w:bottom w:w="55" w:type="Dxa" />
  <w:right w:w="55" w:type="Dxa" />
</w:tblCellMar>"#
        );
    }

    #[test]
    fn test_table_cell_margin_setter() {
        let b = TableCellMargins::new().margin(10, 20, 30, 40).build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:tblCellMar>
  <w:top w:w="10" w:type="Dxa" />
  <w:left w:w="20" w:type="Dxa" />
  <w:bottom w:w="30" w:type="Dxa" />
  <w:right w:w="40" w:type="Dxa" />
</w:tblCellMar>"#
        );
    }
}
