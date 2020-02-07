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

#[derive(Debug, Clone, PartialEq)]
pub struct TableCellBorder {
    position: BorderPosition,
    border_type: BorderType,
    size: usize,
    space: usize,
    color: String,
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
            BorderPosition::IndideH => {
                base.border_inside_h(self.border_type, self.size, self.space, &self.color)
            }
            BorderPosition::IndideV => {
                base.border_inside_v(self.border_type, self.size, self.space, &self.color)
            }
        };
        base.build()
    }
}

#[derive(Debug, Clone, PartialEq)]
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
            inside_h: Some(TableCellBorder::new(BorderPosition::IndideH)),
            inside_v: Some(TableCellBorder::new(BorderPosition::IndideV)),
        }
    }
}

impl TableCellBorders {
    pub fn new() -> TableCellBorders {
        Default::default()
    }

    pub fn set_border(mut self, border: TableCellBorder) -> Self {
        match border.position {
            BorderPosition::Top => self.top = Some(border),
            BorderPosition::Left => self.left = Some(border),
            BorderPosition::Bottom => self.bottom = Some(border),
            BorderPosition::Right => self.right = Some(border),
            BorderPosition::IndideH => self.inside_h = Some(border),
            BorderPosition::IndideV => self.inside_v = Some(border),
        };
        self
    }

    pub fn clear_border(mut self, position: BorderPosition) -> Self {
        match position {
            BorderPosition::Top => self.top = None,
            BorderPosition::Left => self.left = None,
            BorderPosition::Bottom => self.bottom = None,
            BorderPosition::Right => self.right = None,
            BorderPosition::IndideH => self.inside_h = None,
            BorderPosition::IndideV => self.inside_v = None,
        };
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
