use serde::Serialize;

use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TableIndent {
    width: i32,
    width_type: WidthType,
}

impl TableIndent {
    pub fn new(width: i32, width_type: WidthType) -> TableIndent {
        TableIndent { width, width_type }
    }
}

impl BuildXML for TableIndent {
    fn build(&self) -> Vec<u8> {
        XMLBuilder::new().table_indent(20, WidthType::DXA).build()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_table_indent() {
        let b = TableIndent::new(20, WidthType::DXA).build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:tblInd w:w="20" w:type="dxa" />"#
        );
    }
}
