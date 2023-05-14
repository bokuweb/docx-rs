use serde::Serialize;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use super::*;
use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TableCellProperty {
    width: Option<TableCellWidth>,
    borders: Option<TableCellBorders>,
    grid_span: Option<GridSpan>,
    vertical_merge: Option<VMerge>,
    vertical_align: Option<VAlign>,
    text_direction: Option<TextDirection>,
    shading: Option<Shading>,
}

impl TableCellProperty {
    pub fn new() -> TableCellProperty {
        Default::default()
    }

    pub fn width(mut self, v: usize, t: WidthType) -> TableCellProperty {
        self.width = Some(TableCellWidth::new(v, t));
        self
    }

    pub fn vertical_merge(mut self, t: VMergeType) -> TableCellProperty {
        self.vertical_merge = Some(VMerge::new(t));
        self
    }

    pub fn vertical_align(mut self, t: VAlignType) -> TableCellProperty {
        self.vertical_align = Some(VAlign::new(t));
        self
    }

    pub fn text_direction(mut self, t: TextDirectionType) -> Self {
        self.text_direction = Some(TextDirection::new(t));
        self
    }

    pub fn grid_span(mut self, v: usize) -> TableCellProperty {
        self.grid_span = Some(GridSpan::new(v));
        self
    }

    pub fn shading(mut self, s: Shading) -> Self {
        self.shading = Some(s);
        self
    }

    pub fn set_borders(mut self, borders: TableCellBorders) -> Self {
        self.borders = Some(borders);
        self
    }

    pub fn set_border(mut self, border: TableCellBorder) -> Self {
        self.borders = Some(self.borders.unwrap_or_default().set(border));
        self
    }

    pub fn clear_border(mut self, position: TableCellBorderPosition) -> Self {
        self.borders = Some(self.borders.unwrap_or_default().clear(position));
        self
    }

    pub fn clear_all_border(mut self) -> Self {
        self.borders = Some(self.borders.unwrap_or_default().clear_all());
        self
    }
}

impl Default for TableCellProperty {
    fn default() -> Self {
        TableCellProperty {
            width: None,
            borders: None,
            grid_span: None,
            vertical_merge: None,
            vertical_align: None,
            text_direction: None,
            shading: None,
        }
    }
}

impl BuildXML for TableCellProperty {
    fn build(&self) -> Vec<u8> {
        XMLBuilder::new()
            .open_table_cell_property()
            .add_optional_child(&self.width)
            .add_optional_child(&self.borders)
            .add_optional_child(&self.grid_span)
            .add_optional_child(&self.vertical_merge)
            .add_optional_child(&self.vertical_align)
            .add_optional_child(&self.text_direction)
            .add_optional_child(&self.shading)
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

    #[test]
    fn test_grid_span() {
        let c = TableCellProperty::new().grid_span(3);
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:tcPr><w:gridSpan w:val="3" /></w:tcPr>"#
        );
    }

    #[test]
    fn test_vmerge() {
        let c = TableCellProperty::new().vertical_merge(VMergeType::Continue);
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:tcPr><w:vMerge w:val="continue" /></w:tcPr>"#
        );
    }

    #[test]
    fn test_valign() {
        let c = TableCellProperty::new().vertical_align(VAlignType::Center);
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:tcPr><w:vAlign w:val="center" /></w:tcPr>"#
        );
    }

    #[test]
    fn test_shd() {
        let c = TableCellProperty::new()
            .shading(Shading::new().shd_type(ShdType::Clear).fill("FF0000"));
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:tcPr><w:shd w:val="clear" w:color="auto" w:fill="FF0000" /></w:tcPr>"#
        );
    }

    #[test]
    fn test_table_cell_prop_json() {
        let c = TableCellProperty::new()
            .vertical_merge(VMergeType::Continue)
            .grid_span(3)
            .width(200, WidthType::Dxa);
        assert_eq!(
            serde_json::to_string(&c).unwrap(),
            r#"{"width":{"width":200,"widthType":"dxa"},"borders":null,"gridSpan":3,"verticalMerge":"continue","verticalAlign":null,"textDirection":null,"shading":null}"#
        );
    }

    #[test]
    fn test_table_cell_prop_json_with_valign() {
        let c = TableCellProperty::new().vertical_align(VAlignType::Center);
        assert_eq!(
            serde_json::to_string(&c).unwrap(),
            r#"{"width":null,"borders":null,"gridSpan":null,"verticalMerge":null,"verticalAlign":"center","textDirection":null,"shading":null}"#
        );
    }
}
