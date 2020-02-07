use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;

#[derive(Debug, Clone, PartialEq)]
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
        XMLBuilder::new()
            .table_width(self.width, WidthType::DXA)
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
    fn test_table_width() {
        let b = TableWidth::new(20, WidthType::DXA).build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:tblW w:w="20" w:type="dxa" />"#
        );
    }
}
