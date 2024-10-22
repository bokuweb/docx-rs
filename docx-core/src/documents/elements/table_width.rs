use serde::Serialize;

use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TableWidth {
    width: usize,
    width_type: WidthType,
}

impl TableWidth {
    pub fn new(width: usize, width_type: WidthType) -> TableWidth {
        TableWidth { width, width_type }
    }
}

impl BuildXML for TableWidth {
    fn build(&self) -> Vec<u8> {
        XMLBuilder::new(Vec::new())
            .table_width(self.width as i32, self.width_type)
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
    fn test_table_width() {
        let b = TableWidth::new(20, WidthType::Dxa).build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:tblW w:w="20" w:type="dxa" />"#
        );
    }
}
