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
pub struct TableBorder {
    pub border_type: BorderType,
    pub size: usize,
    pub color: String,
    position: BorderPosition,
    space: usize,
}

impl TableBorder {
    pub fn new(position: BorderPosition) -> TableBorder {
        TableBorder {
            position,
            border_type: BorderType::Single,
            size: 2,
            space: 0,
            color: "000000".to_owned(),
        }
    }

    pub fn color(mut self, color: impl Into<String>) -> TableBorder {
        self.color = color.into();
        self
    }

    pub fn size(mut self, size: usize) -> TableBorder {
        self.size = size;
        self
    }

    pub fn border_type(mut self, border_type: BorderType) -> TableBorder {
        self.border_type = border_type;
        self
    }
}

impl BuildXML for TableBorder {
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
pub struct TableBorders {
    top: Option<TableBorder>,
    left: Option<TableBorder>,
    bottom: Option<TableBorder>,
    right: Option<TableBorder>,
    inside_h: Option<TableBorder>,
    inside_v: Option<TableBorder>,
}

impl Default for TableBorders {
    fn default() -> TableBorders {
        TableBorders {
            top: Some(TableBorder::new(BorderPosition::Top)),
            left: Some(TableBorder::new(BorderPosition::Left)),
            bottom: Some(TableBorder::new(BorderPosition::Bottom)),
            right: Some(TableBorder::new(BorderPosition::Right)),
            inside_h: Some(TableBorder::new(BorderPosition::InsideH)),
            inside_v: Some(TableBorder::new(BorderPosition::InsideV)),
        }
    }
}

impl TableBorders {
    pub fn new() -> TableBorders {
        Default::default()
    }

    pub fn with_empty() -> TableBorders {
        TableBorders {
            top: None,
            left: None,
            bottom: None,
            right: None,
            inside_h: None,
            inside_v: None,
        }
    }

    pub fn set(mut self, border: TableBorder) -> Self {
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
        let nil = TableBorder::new(position.clone()).border_type(BorderType::Nil);
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
        self.top = Some(TableBorder::new(BorderPosition::Top).border_type(BorderType::Nil));
        self.left = Some(TableBorder::new(BorderPosition::Left).border_type(BorderType::Nil));
        self.bottom = Some(TableBorder::new(BorderPosition::Bottom).border_type(BorderType::Nil));
        self.right = Some(TableBorder::new(BorderPosition::Right).border_type(BorderType::Nil));
        self.inside_h =
            Some(TableBorder::new(BorderPosition::InsideH).border_type(BorderType::Nil));
        self.inside_v =
            Some(TableBorder::new(BorderPosition::InsideV).border_type(BorderType::Nil));
        self
    }
}

impl BuildXML for TableBorders {
    fn build(&self) -> Vec<u8> {
        XMLBuilder::new()
            .open_table_borders()
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

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_table_borders() {
        let b = TableBorders::new().build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:tblBorders><w:top w:val="single" w:sz="2" w:space="0" w:color="000000" /><w:left w:val="single" w:sz="2" w:space="0" w:color="000000" /><w:bottom w:val="single" w:sz="2" w:space="0" w:color="000000" /><w:right w:val="single" w:sz="2" w:space="0" w:color="000000" /><w:insideH w:val="single" w:sz="2" w:space="0" w:color="000000" /><w:insideV w:val="single" w:sz="2" w:space="0" w:color="000000" /></w:tblBorders>"#
        );
    }

    #[test]
    fn test_table_borders_set() {
        let b = TableBorders::new()
            .set(TableBorder::new(BorderPosition::Left).color("AAAAAA"))
            .clear(BorderPosition::Top)
            .build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:tblBorders><w:top w:val="nil" w:sz="2" w:space="0" w:color="000000" /><w:left w:val="single" w:sz="2" w:space="0" w:color="AAAAAA" /><w:bottom w:val="single" w:sz="2" w:space="0" w:color="000000" /><w:right w:val="single" w:sz="2" w:space="0" w:color="000000" /><w:insideH w:val="single" w:sz="2" w:space="0" w:color="000000" /><w:insideV w:val="single" w:sz="2" w:space="0" w:color="000000" /></w:tblBorders>"#
        );
    }
}
