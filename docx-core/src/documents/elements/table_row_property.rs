use serde::Serialize;
use std::io::Write;

use super::*;
use crate::xml_builder::*;
use crate::{documents::BuildXML, HeightRule};

#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TableRowProperty {
    // 1. w:cnfStyle
    // TODO: Add CnfStyle type
    // 2. w:divId
    // TODO: Add DivId type
    // 3. w:gridBefore
    grid_before: Option<u32>,
    // 4. w:gridAfter
    grid_after: Option<u32>,
    // 5. w:wBefore
    width_before: Option<f32>,
    // 6. w:wAfter
    width_after: Option<f32>,
    // 7. w:cantSplit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cant_split: Option<CantSplit>,
    // 8. w:trHeight
    #[serde(skip_serializing_if = "Option::is_none")]
    row_height: Option<f32>,
    // 9. w:tblHeader
    // TODO: Add TblHeader type
    // 10. w:tblCellSpacing
    // TODO: Add TblCellSpacing type
    // 11. w:jc
    // TODO: Add Jc type
    // 12. w:hidden
    // TODO: Add Hidden type
    // 13. w:ins
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ins: Option<Insert>,
    // 14. w:del
    #[serde(skip_serializing_if = "Option::is_none")]
    pub del: Option<Delete>,
    // 15. w:trPrChange
    // TODO: Add TrPrChange type
    #[serde(skip_serializing_if = "Option::is_none")]
    height_rule: Option<HeightRule>,
}

impl TableRowProperty {
    pub fn new() -> TableRowProperty {
        Default::default()
    }

    pub fn grid_after(mut self, after: u32) -> Self {
        self.grid_after = Some(after);
        self
    }

    pub fn width_after(mut self, w: f32) -> Self {
        self.width_after = Some(w);
        self
    }

    pub fn grid_before(mut self, before: u32) -> Self {
        self.grid_before = Some(before);
        self
    }

    pub fn width_before(mut self, w: f32) -> Self {
        self.width_before = Some(w);
        self
    }

    pub fn row_height(mut self, h: f32) -> Self {
        self.row_height = Some(h);
        self
    }

    pub fn height_rule(mut self, r: HeightRule) -> Self {
        self.height_rule = Some(r);
        self
    }

    pub fn delete(mut self, d: Delete) -> Self {
        self.del = Some(d);
        self
    }

    pub fn insert(mut self, i: Insert) -> Self {
        self.ins = Some(i);
        self
    }

    pub fn cant_split(mut self) -> Self {
        self.cant_split = Some(CantSplit::default());
        self
    }
}

impl BuildXML for TableRowProperty {
    // <<<<<<< HEAD
    //     fn build(&self) -> Vec<u8> {
    //         let mut b = XMLBuilder::new()
    //             .open_table_row_property()
    //             .add_optional_child(&self.cant_split);
    //
    //         if let Some(h) = self.row_height {
    //             b = b.table_row_height(
    //                 &format!("{}", h),
    //                 &self.height_rule.unwrap_or_default().to_string(),
    //             )
    //         }
    //
    //         b = b
    //             .add_optional_child(&self.del)
    //             .add_optional_child(&self.ins);
    //
    //         b.close().build()
    fn build_to<W: Write>(
        &self,
        stream: xml::writer::EventWriter<W>,
    ) -> xml::writer::Result<xml::writer::EventWriter<W>> {
        //         // TODO remaining elements to be added in XML
        XMLBuilder::from(stream)
            .open_table_row_property()?
            .add_optional_child(&self.cant_split)?
            .apply_opt(self.row_height, |h, b| {
                b.table_row_height(
                    &format!("{}", h),
                    &self.height_rule.unwrap_or_default().to_string(),
                )
            })?
            .add_optional_child(&self.del)?
            .add_optional_child(&self.ins)?
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
    fn test_default() {
        let b = TableRowProperty::new().build();
        assert_eq!(str::from_utf8(&b).unwrap(), r#"<w:trPr />"#);
    }

    #[test]
    fn test_cant_split() {
        let b = TableRowProperty::new().cant_split().build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:trPr><w:cantSplit /></w:trPr>"#
        );
    }
}
