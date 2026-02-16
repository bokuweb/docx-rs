use serde::Serialize;
use std::io::Write;

use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;
use crate::CellMargin;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TableCellMargins {
    top: CellMargin,
    left: CellMargin,
    bottom: CellMargin,
    right: CellMargin,
}

impl Default for TableCellMargins {
    fn default() -> TableCellMargins {
        TableCellMargins {
            top: CellMargin {
                val: 0,
                width_type: WidthType::Dxa,
            },
            left: CellMargin::default(),
            bottom: CellMargin {
                val: 0,
                width_type: WidthType::Dxa,
            },
            right: CellMargin::default(),
        }
    }
}

impl TableCellMargins {
    pub fn new() -> TableCellMargins {
        Default::default()
    }

    pub fn margin(self, top: usize, right: usize, bottom: usize, left: usize) -> TableCellMargins {
        TableCellMargins {
            top: CellMargin::new(top, WidthType::Dxa),
            left: CellMargin::new(left, WidthType::Dxa),
            bottom: CellMargin::new(bottom, WidthType::Dxa),
            right: CellMargin::new(right, WidthType::Dxa),
        }
    }

    pub fn margin_top(mut self, v: usize, t: WidthType) -> Self {
        self.top = CellMargin::new(v, t);
        self
    }

    pub fn margin_right(mut self, v: usize, t: WidthType) -> Self {
        self.right = CellMargin::new(v, t);
        self
    }

    pub fn margin_left(mut self, v: usize, t: WidthType) -> Self {
        self.left = CellMargin::new(v, t);
        self
    }

    pub fn margin_bottom(mut self, v: usize, t: WidthType) -> Self {
        self.bottom = CellMargin::new(v, t);
        self
    }
}

impl BuildXML for TableCellMargins {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_table_cell_margins()?
            .margin_top(self.top.val as i32, self.top.width_type)?
            .margin_left(self.left.val as i32, self.left.width_type)?
            .margin_bottom(self.bottom.val as i32, self.bottom.width_type)?
            .margin_right(self.right.val as i32, self.right.width_type)?
            .close()?
            .into_inner()
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
            r#"<w:tblCellMar><w:top w:w="0" w:type="dxa" /><w:left w:w="55" w:type="dxa" /><w:bottom w:w="0" w:type="dxa" /><w:right w:w="55" w:type="dxa" /></w:tblCellMar>"#
        );
    }

    #[test]
    fn test_table_cell_margin_setter() {
        let b = TableCellMargins::new().margin(10, 20, 30, 40).build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:tblCellMar><w:top w:w="10" w:type="dxa" /><w:left w:w="40" w:type="dxa" /><w:bottom w:w="30" w:type="dxa" /><w:right w:w="20" w:type="dxa" /></w:tblCellMar>"#
        );
    }
}
