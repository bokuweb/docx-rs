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

    pub fn margin(self, top: usize, right: usize, bottom: usize, left: usize) -> TableCellMargins {
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
            .margin_top(self.top as i32, WidthType::DXA)
            .margin_left(self.left as i32, WidthType::DXA)
            .margin_bottom(self.bottom as i32, WidthType::DXA)
            .margin_right(self.right as i32, WidthType::DXA)
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
  <w:top w:w="55" w:type="dxa" />
  <w:left w:w="54" w:type="dxa" />
  <w:bottom w:w="55" w:type="dxa" />
  <w:right w:w="55" w:type="dxa" />
</w:tblCellMar>"#
        );
    }

    #[test]
    fn test_table_cell_margin_setter() {
        let b = TableCellMargins::new().margin(10, 20, 30, 40).build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:tblCellMar>
  <w:top w:w="10" w:type="dxa" />
  <w:left w:w="40" w:type="dxa" />
  <w:bottom w:w="30" w:type="dxa" />
  <w:right w:w="20" w:type="dxa" />
</w:tblCellMar>"#
        );
    }
}
