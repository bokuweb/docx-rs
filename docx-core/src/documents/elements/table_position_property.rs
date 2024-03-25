use serde::Serialize;

use crate::documents::BuildXML;
use crate::xml_builder::*;

/// https://learn.microsoft.com/en-us/dotnet/api/documentformat.openxml.wordprocessing.tablepositionproperties?view=openxml-3.0.1
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[serde(rename_all = "camelCase")]
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
    pub table_position_x_alignment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_position_y_alignment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_position_x: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_position_y: Option<i32>,
}

impl TablePositionProperty {
    pub fn new() -> TablePositionProperty {
        Default::default()
    }
}

impl BuildXML for TablePositionProperty {
    fn build(&self) -> Vec<u8> {
        XMLBuilder::new().table_position_property(self).build()
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
}
