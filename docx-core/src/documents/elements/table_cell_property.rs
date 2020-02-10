use serde::Serialize;

use super::{GridSpan, TableCellBorders, TableCellWidth, VMerge};
use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;

#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TableCellProperty {
    width: Option<TableCellWidth>,
    borders: Option<TableCellBorders>,
    grid_span: Option<GridSpan>,
    vertical_merge: Option<VMerge>,
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

    pub fn grid_span(mut self, v: usize) -> TableCellProperty {
        self.grid_span = Some(GridSpan::new(v));
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
    fn test_table_cell_prop_json() {
        let c = TableCellProperty::new()
            .vertical_merge(VMergeType::Continue)
            .grid_span(3)
            .width(200, WidthType::DXA);
        assert_eq!(
            serde_json::to_string(&c).unwrap(),
            r#"{"width":{"width":200,"widthType":"DXA"},"borders":null,"gridSpan":3,"verticalMerge":"continue"}"#
        );
    }
}
