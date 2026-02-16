use serde::Serialize;
use std::io::Write;

use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CellMargin {
    pub val: usize,
    pub width_type: WidthType,
}

impl CellMargin {
    pub fn new(val: usize, t: WidthType) -> Self {
        Self { val, width_type: t }
    }
}

impl Default for CellMargin {
    fn default() -> CellMargin {
        CellMargin {
            val: 55,
            width_type: WidthType::Dxa,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CellMargins {
    #[serde(skip_serializing_if = "Option::is_none")]
    top: Option<CellMargin>,
    #[serde(skip_serializing_if = "Option::is_none")]
    left: Option<CellMargin>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bottom: Option<CellMargin>,
    #[serde(skip_serializing_if = "Option::is_none")]
    right: Option<CellMargin>,
}

impl CellMargins {
    pub fn new() -> CellMargins {
        Default::default()
    }

    pub fn margin_top(mut self, v: usize, t: WidthType) -> Self {
        self.top = Some(CellMargin::new(v, t));
        self
    }

    pub fn margin_right(mut self, v: usize, t: WidthType) -> Self {
        self.right = Some(CellMargin::new(v, t));
        self
    }

    pub fn margin_left(mut self, v: usize, t: WidthType) -> Self {
        self.left = Some(CellMargin::new(v, t));
        self
    }

    pub fn margin_bottom(mut self, v: usize, t: WidthType) -> Self {
        self.bottom = Some(CellMargin::new(v, t));
        self
    }
}

impl BuildXML for CellMargins {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_cell_margins()?
            .apply_opt(self.top.as_ref(), |top, b| {
                b.margin_top(top.val as i32, top.width_type)
            })?
            .apply_opt(self.left.as_ref(), |left, b| {
                b.margin_left(left.val as i32, left.width_type)
            })?
            .apply_opt(self.bottom.as_ref(), |bottom, b| {
                b.margin_bottom(bottom.val as i32, bottom.width_type)
            })?
            .apply_opt(self.right.as_ref(), |right, b| {
                b.margin_right(right.val as i32, right.width_type)
            })?
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
    fn test_cell_margin() {
        let b = CellMargins::new().margin_top(10, WidthType::Dxa).build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:tcMar><w:top w:w="10" w:type="dxa" /></w:tcMar>"#
        );
    }
}
