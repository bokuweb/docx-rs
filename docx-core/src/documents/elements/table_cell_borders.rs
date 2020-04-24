use serde::Serialize;

use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;

/*
    Please see. L.4.3.2.18 Cell Border Properties

    left – left border
    right – right border
    top – top border
    bottom – bottom border
    insideH – inner horizontal borders
    insideV – inner vertical borders
    tl2br – diagonal border from top left corner to bottom right corner
    tr2bl – diagonal border from top right corner to bottom left corner
*/
#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TableCellBorder {
    pub border_type: BorderType,
    pub size: usize,
    pub color: String,
    position: BorderPosition,
    space: usize,
}

impl TableCellBorder {
    pub fn new(position: BorderPosition) -> TableCellBorder {
        TableCellBorder {
            position,
            border_type: BorderType::Single,
            size: 2,
            space: 0,
            color: "000000".to_owned(),
        }
    }

    pub fn color(mut self, color: impl Into<String>) -> TableCellBorder {
        self.color = color.into();
        self
    }

    pub fn size(mut self, size: usize) -> TableCellBorder {
        self.size = size;
        self
    }

    pub fn border_type(mut self, border_type: BorderType) -> TableCellBorder {
        self.border_type = border_type;
        self
    }
}

impl BuildXML for TableCellBorder {
    fn build(&self) -> Vec<u8> {
        let base = XMLBuilder::new();
        let base = match self.position {
            BorderPosition::Top => {
                base.border_top(self.border_type, self.size, self.space, &self.color)
            }
            BorderPosition::Left => {
                base.border_left(self.border_type, self.size, self.space, &self.color)
            }
            BorderPosition::Bottom => {
                base.border_bottom(self.border_type, self.size, self.space, &self.color)
            }
            BorderPosition::Right => {
                base.border_right(self.border_type, self.size, self.space, &self.color)
            }
            BorderPosition::InsideH => {
                base.border_inside_h(self.border_type, self.size, self.space, &self.color)
            }
            BorderPosition::InsideV => {
                base.border_inside_v(self.border_type, self.size, self.space, &self.color)
            }
        };
        base.build()
    }
}

#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TableCellBorders {
    top: Option<TableCellBorder>,
    left: Option<TableCellBorder>,
    bottom: Option<TableCellBorder>,
    right: Option<TableCellBorder>,
    inside_h: Option<TableCellBorder>,
    inside_v: Option<TableCellBorder>,
}

impl Default for TableCellBorders {
    fn default() -> TableCellBorders {
        TableCellBorders {
            top: Some(TableCellBorder::new(BorderPosition::Top)),
            left: Some(TableCellBorder::new(BorderPosition::Left)),
            bottom: Some(TableCellBorder::new(BorderPosition::Bottom)),
            right: Some(TableCellBorder::new(BorderPosition::Right)),
            inside_h: Some(TableCellBorder::new(BorderPosition::InsideH)),
            inside_v: Some(TableCellBorder::new(BorderPosition::InsideV)),
        }
    }
}

impl TableCellBorders {
    pub fn new() -> TableCellBorders {
        Default::default()
    }

    pub fn set(mut self, border: TableCellBorder) -> Self {
        match border.position {
            BorderPosition::Top => self.top = Some(border),
            BorderPosition::Left => self.left = Some(border),
            BorderPosition::Bottom => self.bottom = Some(border),
            BorderPosition::Right => self.right = Some(border),
            BorderPosition::InsideH => self.inside_h = Some(border),
            BorderPosition::InsideV => self.inside_v = Some(border),
        };
        self
    }

    pub fn clear(mut self, position: BorderPosition) -> Self {
        let nil = TableCellBorder::new(position.clone()).border_type(BorderType::Nil);
        match position {
            BorderPosition::Top => self.top = Some(nil),
            BorderPosition::Left => self.left = Some(nil),
            BorderPosition::Bottom => self.bottom = Some(nil),
            BorderPosition::Right => self.right = Some(nil),
            BorderPosition::InsideH => self.inside_h = Some(nil),
            BorderPosition::InsideV => self.inside_v = Some(nil),
        };
        self
    }

    pub fn clear_all(mut self) -> Self {
        self.top = Some(TableCellBorder::new(BorderPosition::Top).border_type(BorderType::Nil));
        self.left = Some(TableCellBorder::new(BorderPosition::Left).border_type(BorderType::Nil));
        self.bottom =
            Some(TableCellBorder::new(BorderPosition::Bottom).border_type(BorderType::Nil));
        self.right = Some(TableCellBorder::new(BorderPosition::Right).border_type(BorderType::Nil));
        self.inside_h =
            Some(TableCellBorder::new(BorderPosition::InsideH).border_type(BorderType::Nil));
        self.inside_v =
            Some(TableCellBorder::new(BorderPosition::InsideV).border_type(BorderType::Nil));
        self
    }
}

impl BuildXML for TableCellBorders {
    fn build(&self) -> Vec<u8> {
        XMLBuilder::new()
            .open_table_cell_borders()
            .add_optional_child(&self.top)
            .add_optional_child(&self.left)
            .add_optional_child(&self.bottom)
            .add_optional_child(&self.right)
            .add_optional_child(&self.inside_h)
            .add_optional_child(&self.inside_v)
            .close()
            .build()
    }
}
