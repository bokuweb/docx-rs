use serde::Serialize;

use super::{Justification, TableBorders, TableCellMargins, TableIndent, TableWidth};
use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TableProperty {
    width: TableWidth,
    justification: Justification,
    borders: TableBorders,
    margins: TableCellMargins,
    indent: Option<TableIndent>,
}

impl Default for TableProperty {
    fn default() -> Self {
        TableProperty {
            width: TableWidth::new(0, WidthType::Auto),
            justification: Justification::new("left"),
            borders: TableBorders::new(),
            margins: TableCellMargins::new(),
            indent: None,
        }
    }
}

impl TableProperty {
    pub fn new() -> TableProperty {
        Default::default()
    }

    pub fn indent(mut self, v: usize) -> TableProperty {
        self.indent = Some(TableIndent::new(v, WidthType::DXA));
        self
    }

    pub fn width(mut self, v: usize, t: WidthType) -> TableProperty {
        self.width = TableWidth::new(v, t);
        self
    }

    pub fn align(mut self, v: TableAlignmentType) -> TableProperty {
        self.justification = Justification::new(v.to_string());
        self
    }
}

impl BuildXML for TableProperty {
    fn build(&self) -> Vec<u8> {
        XMLBuilder::new()
            .open_table_property()
            .add_child(&self.width)
            .add_child(&self.justification)
            .add_child(&self.borders)
            .add_child(&self.margins)
            .add_optional_child(&self.indent)
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
        let c = TableProperty::new();
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:tblPr><w:tblW w:w="0" w:type="dxa" /><w:jc w:val="left" /><w:tblBorders><w:top w:val="single" w:sz="2" w:space="0" w:color="000000" /><w:left w:val="single" w:sz="2" w:space="0" w:color="000000" /><w:bottom w:val="single" w:sz="2" w:space="0" w:color="000000" /><w:right w:val="single" w:sz="2" w:space="0" w:color="000000" /><w:insideH w:val="single" w:sz="2" w:space="0" w:color="000000" /><w:insideV w:val="single" w:sz="2" w:space="0" w:color="000000" /></w:tblBorders><w:tblCellMar>
  <w:top w:w="55" w:type="dxa" />
  <w:left w:w="54" w:type="dxa" />
  <w:bottom w:w="55" w:type="dxa" />
  <w:right w:w="55" w:type="dxa" />
</w:tblCellMar></w:tblPr>"#
        );
    }

    #[test]
    fn test_table_property_json() {
        let p = TableProperty::new().indent(100);
        assert_eq!(
            serde_json::to_string(&p).unwrap(),
            r#"{"width":{"width":0,"widthType":"Auto"},"justification":"left","borders":{"top":{"position":"top","borderType":"single","size":2,"space":0,"color":"000000"},"left":{"position":"left","borderType":"single","size":2,"space":0,"color":"000000"},"bottom":{"position":"bottom","borderType":"single","size":2,"space":0,"color":"000000"},"right":{"position":"right","borderType":"single","size":2,"space":0,"color":"000000"},"insideH":{"position":"insideH","borderType":"single","size":2,"space":0,"color":"000000"},"insideV":{"position":"insideV","borderType":"single","size":2,"space":0,"color":"000000"}},"margins":{"top":55,"left":54,"bottom":55,"right":55},"indent":{"width":100,"widthType":"DXA"}}"#
        );
    }
}
