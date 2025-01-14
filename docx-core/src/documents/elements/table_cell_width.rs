use serde::Serialize;
use std::io::Write;

use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;

#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TableCellWidth {
    width: usize,
    width_type: WidthType,
}

impl TableCellWidth {
    pub fn new(width: usize, width_type: WidthType) -> TableCellWidth {
        TableCellWidth { width, width_type }
    }
}

impl BuildXML for TableCellWidth {
    fn build_to<W: Write>(
        &self,
        stream: xml::writer::EventWriter<W>,
    ) -> xml::writer::Result<xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .table_cell_width(self.width as i32, self.width_type)?
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
        let b = TableCellWidth::new(20, WidthType::Dxa).build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:tcW w:w="20" w:type="dxa" />"#
        );
    }
}
