use serde::Serialize;

use crate::documents::BuildXML;
use crate::xml_builder::*;

/// https://learn.microsoft.com/en-us/dotnet/api/documentformat.openxml.wordprocessing.tablepositionproperties?view=openxml-3.0.1
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "wasm", derive(ts_rs::TS), ts(export))]
pub struct TablePositionProperty {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_from_text: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_from_text: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_anchor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_anchor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_x_alignment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_y_alignment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_x: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_y: Option<i32>,
}

impl TablePositionProperty {
    pub fn new() -> TablePositionProperty {
        Default::default()
    }

    pub fn left_from_text(mut self, v: i32) -> Self {
        self.left_from_text = Some(v);
        self
    }

    pub fn right_from_text(mut self, v: i32) -> Self {
        self.right_from_text = Some(v);
        self
    }

    pub fn vertical_anchor(mut self, v: impl Into<String>) -> Self {
        self.vertical_anchor = Some(v.into());
        self
    }

    pub fn horizontal_anchor(mut self, v: impl Into<String>) -> Self {
        self.horizontal_anchor = Some(v.into());
        self
    }

    pub fn position_x_alignment(mut self, v: impl Into<String>) -> Self {
        self.position_x_alignment = Some(v.into());
        self
    }

    pub fn position_y_alignment(mut self, v: impl Into<String>) -> Self {
        self.position_y_alignment = Some(v.into());
        self
    }

    pub fn position_x(mut self, v: i32) -> Self {
        self.position_x = Some(v);
        self
    }

    pub fn position_y(mut self, v: i32) -> Self {
        self.position_y = Some(v);
        self
    }
}

impl BuildXML for TablePositionProperty {
    fn build(&self) -> Vec<u8> {
        XMLBuilder::new(Vec::new())
            .table_position_property(self)
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
        let b = TablePositionProperty::new().build();
        assert_eq!(str::from_utf8(&b).unwrap(), r#"<w:tblpPr />"#);
    }

    #[test]
    fn test_some_attrs() {
        let b = TablePositionProperty::new()
            .left_from_text(142)
            .right_from_text(142)
            .vertical_anchor("text")
            .horizontal_anchor("margin")
            .position_x_alignment("right")
            .position_y(511)
            .build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:tblpPr w:leftFromText="142" w:rightFromText="142" w:vertAnchor="text" w:horzAnchor="margin" w:tblpXSpec="right" w:tblpY="511" />"#
        );
    }
}
