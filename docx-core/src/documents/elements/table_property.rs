use serde::Serialize;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use super::*;
use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TableProperty {
    width: TableWidth,
    justification: Justification,
    borders: TableBorders,
    #[serde(skip_serializing_if = "Option::is_none")]
    margins: Option<TableCellMargins>,
    #[serde(skip_serializing_if = "Option::is_none")]
    indent: Option<TableIndent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<TableStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    layout: Option<TableLayout>,
}

impl Default for TableProperty {
    fn default() -> Self {
        TableProperty {
            width: TableWidth::new(0, WidthType::Auto),
            justification: Justification::new("left"),
            borders: TableBorders::new(),
            margins: None,
            indent: None,
            style: None,
            layout: None,
        }
    }
}

impl TableProperty {
    pub fn new() -> TableProperty {
        Default::default()
    }

    pub fn without_borders() -> TableProperty {
        TableProperty {
            borders: TableBorders::with_empty(),
            ..Default::default()
        }
    }

    pub fn indent(mut self, v: i32) -> TableProperty {
        self.indent = Some(TableIndent::new(v, WidthType::Dxa));
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

    pub fn set_margins(mut self, margins: TableCellMargins) -> Self {
        self.margins = Some(margins);
        self
    }

    pub fn cell_margin_top(mut self, v: usize, t: WidthType) -> Self {
        if let Some(margins) = self.margins {
            self.margins = Some(margins.margin_top(v, t));
        } else {
            let margins = TableCellMargins::new();
            self.margins = Some(margins.margin_top(v, t));
        }
        self
    }

    pub fn cell_margin_right(mut self, v: usize, t: WidthType) -> Self {
        if let Some(margins) = self.margins {
            self.margins = Some(margins.margin_right(v, t));
        } else {
            let margins = TableCellMargins::new();
            self.margins = Some(margins.margin_right(v, t));
        }
        self
    }

    pub fn cell_margin_bottom(mut self, v: usize, t: WidthType) -> Self {
        if let Some(margins) = self.margins {
            self.margins = Some(margins.margin_bottom(v, t));
        } else {
            let margins = TableCellMargins::new();
            self.margins = Some(margins.margin_bottom(v, t));
        }
        self
    }

    pub fn cell_margin_left(mut self, v: usize, t: WidthType) -> Self {
        if let Some(margins) = self.margins {
            self.margins = Some(margins.margin_left(v, t));
        } else {
            let margins = TableCellMargins::new();
            self.margins = Some(margins.margin_left(v, t));
        }
        self
    }

    pub fn set_borders(mut self, borders: TableBorders) -> Self {
        self.borders = borders;
        self
    }

    pub fn set_border(mut self, border: TableBorder) -> Self {
        self.borders = self.borders.set(border);
        self
    }

    pub fn clear_border(mut self, position: TableBorderPosition) -> Self {
        self.borders = self.borders.clear(position);
        self
    }

    pub fn clear_all_border(mut self) -> Self {
        self.borders = self.borders.clear_all();
        self
    }

    pub fn style(mut self, s: impl Into<String>) -> Self {
        self.style = Some(TableStyle::new(s));
        self
    }

    pub fn layout(mut self, t: TableLayoutType) -> Self {
        self.layout = Some(TableLayout::new(t));
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
            .add_optional_child(&self.margins)
            .add_optional_child(&self.indent)
            .add_optional_child(&self.style)
            .add_optional_child(&self.layout)
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
            r#"<w:tblPr><w:tblW w:w="0" w:type="dxa" /><w:jc w:val="left" /><w:tblBorders><w:top w:val="single" w:sz="2" w:space="0" w:color="000000" /><w:left w:val="single" w:sz="2" w:space="0" w:color="000000" /><w:bottom w:val="single" w:sz="2" w:space="0" w:color="000000" /><w:right w:val="single" w:sz="2" w:space="0" w:color="000000" /><w:insideH w:val="single" w:sz="2" w:space="0" w:color="000000" /><w:insideV w:val="single" w:sz="2" w:space="0" w:color="000000" /></w:tblBorders></w:tblPr>"#
        );
    }

    #[test]
    fn test_table_property_json() {
        let p = TableProperty::new().indent(100);
        assert_eq!(
            serde_json::to_string(&p).unwrap(),
            r#"{"width":{"width":0,"widthType":"auto"},"justification":"left","borders":{"top":{"borderType":"single","size":2,"color":"000000","position":"top","space":0},"left":{"borderType":"single","size":2,"color":"000000","position":"left","space":0},"bottom":{"borderType":"single","size":2,"color":"000000","position":"bottom","space":0},"right":{"borderType":"single","size":2,"color":"000000","position":"right","space":0},"insideH":{"borderType":"single","size":2,"color":"000000","position":"insideH","space":0},"insideV":{"borderType":"single","size":2,"color":"000000","position":"insideV","space":0}},"indent":{"width":100,"widthType":"dxa"}}"#
        );
    }
}
