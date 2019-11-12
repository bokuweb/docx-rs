use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone)]
pub struct TableRowProperty {}

impl TableRowProperty {
    pub fn new() -> TableRowProperty {
        TableRowProperty {}
    }
}

impl BuildXML for TableRowProperty {
    fn build(&self) -> Vec<u8> {
        XMLBuilder::new().open_table_row_property().close().build()
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
}
