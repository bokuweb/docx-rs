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
    position: TableCellBorderPosition,
    space: usize,
}

impl TableCellBorder {
    pub fn new(position: TableCellBorderPosition) -> TableCellBorder {
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

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn get_color(&self) -> String {
        self.color.clone()
    }

    pub fn get_border_type(&self) -> BorderType {
        self.border_type
    }
}

impl BuildXML for TableCellBorder {
    fn build(&self) -> Vec<u8> {
        let base = XMLBuilder::new();
        let base = match self.position {
            TableCellBorderPosition::Top => {
                base.border_top(self.border_type, self.size, self.space, &self.color)
            }
            TableCellBorderPosition::Left => {
                base.border_left(self.border_type, self.size, self.space, &self.color)
            }
            TableCellBorderPosition::Bottom => {
                base.border_bottom(self.border_type, self.size, self.space, &self.color)
            }
            TableCellBorderPosition::Right => {
                base.border_right(self.border_type, self.size, self.space, &self.color)
            }
            TableCellBorderPosition::InsideH => {
                base.border_inside_h(self.border_type, self.size, self.space, &self.color)
            }
            TableCellBorderPosition::InsideV => {
                base.border_inside_v(self.border_type, self.size, self.space, &self.color)
            }
            TableCellBorderPosition::Tr2bl => {
                base.border_tr2bl(self.border_type, self.size, self.space, &self.color)
            }
            TableCellBorderPosition::Tl2br => {
                base.border_tl2br(self.border_type, self.size, self.space, &self.color)
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
    tr2bl: Option<TableCellBorder>,
    tl2br: Option<TableCellBorder>,
}

impl Default for TableCellBorders {
    fn default() -> TableCellBorders {
        TableCellBorders {
            top: Some(TableCellBorder::new(TableCellBorderPosition::Top)),
            left: Some(TableCellBorder::new(TableCellBorderPosition::Left)),
            bottom: Some(TableCellBorder::new(TableCellBorderPosition::Bottom)),
            right: Some(TableCellBorder::new(TableCellBorderPosition::Right)),
            inside_h: Some(TableCellBorder::new(TableCellBorderPosition::InsideH)),
            inside_v: Some(TableCellBorder::new(TableCellBorderPosition::InsideV)),
            tr2bl: None,
            tl2br: None,
        }
    }
}

impl TableCellBorders {
    pub fn new() -> TableCellBorders {
        Default::default()
    }

    pub fn with_empty() -> TableCellBorders {
        TableCellBorders {
            top: None,
            left: None,
            bottom: None,
            right: None,
            inside_h: None,
            inside_v: None,
            tr2bl: None,
            tl2br: None,
        }
    }

    pub fn set(mut self, border: TableCellBorder) -> Self {
        match border.position {
            TableCellBorderPosition::Top => self.top = Some(border),
            TableCellBorderPosition::Left => self.left = Some(border),
            TableCellBorderPosition::Bottom => self.bottom = Some(border),
            TableCellBorderPosition::Right => self.right = Some(border),
            TableCellBorderPosition::InsideH => self.inside_h = Some(border),
            TableCellBorderPosition::InsideV => self.inside_v = Some(border),
            TableCellBorderPosition::Tr2bl => self.tr2bl = Some(border),
            TableCellBorderPosition::Tl2br => self.tl2br = Some(border),
        };
        self
    }

    pub fn clear(mut self, position: TableCellBorderPosition) -> Self {
        let nil = TableCellBorder::new(position.clone()).border_type(BorderType::Nil);
        match position {
            TableCellBorderPosition::Top => self.top = Some(nil),
            TableCellBorderPosition::Left => self.left = Some(nil),
            TableCellBorderPosition::Bottom => self.bottom = Some(nil),
            TableCellBorderPosition::Right => self.right = Some(nil),
            TableCellBorderPosition::InsideH => self.inside_h = Some(nil),
            TableCellBorderPosition::InsideV => self.inside_v = Some(nil),
            TableCellBorderPosition::Tr2bl => self.tr2bl = Some(nil),
            TableCellBorderPosition::Tl2br => self.tl2br = Some(nil),
        };
        self
    }

    pub fn clear_all(mut self) -> Self {
        self.top =
            Some(TableCellBorder::new(TableCellBorderPosition::Top).border_type(BorderType::Nil));
        self.left =
            Some(TableCellBorder::new(TableCellBorderPosition::Left).border_type(BorderType::Nil));
        self.bottom = Some(
            TableCellBorder::new(TableCellBorderPosition::Bottom).border_type(BorderType::Nil),
        );
        self.right =
            Some(TableCellBorder::new(TableCellBorderPosition::Right).border_type(BorderType::Nil));
        self.inside_h = Some(
            TableCellBorder::new(TableCellBorderPosition::InsideH).border_type(BorderType::Nil),
        );
        self.inside_v = Some(
            TableCellBorder::new(TableCellBorderPosition::InsideV).border_type(BorderType::Nil),
        );
        self.tl2br =
            Some(TableCellBorder::new(TableCellBorderPosition::Tl2br).border_type(BorderType::Nil));
        self.tr2bl =
            Some(TableCellBorder::new(TableCellBorderPosition::Tr2bl).border_type(BorderType::Nil));
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
            .add_optional_child(&self.tl2br)
            .add_optional_child(&self.tr2bl)
            .close()
            .build()
    }
}
