use serde::Serialize;

use super::*;
use crate::xml_builder::*;
use crate::{documents::BuildXML, HeightRule};

#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TableRowProperty {
    grid_after: Option<u32>,
    width_after: Option<f32>,
    grid_before: Option<u32>,
    width_before: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    row_height: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    height_rule: Option<HeightRule>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub del: Option<Delete>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ins: Option<Insert>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cant_split: Option<CantSplit>,
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
    fn build(&self) -> Vec<u8> {
        let mut b = XMLBuilder::new()
            .open_table_row_property()
            .add_optional_child(&self.del)
            .add_optional_child(&self.ins)
            .add_optional_child(&self.cant_split);
        if let Some(h) = self.row_height {
            b = b.table_row_height(
                &format!("{}", h),
                &self.height_rule.unwrap_or_default().to_string(),
            )
        }
        b.close().build()
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
