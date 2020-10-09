use serde::Serialize;

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct TableRowProperty {
    grid_after: Option<u32>,
    width_after: Option<f32>,
    row_height: Option<f32>,
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

    pub fn row_height(mut self, h: f32) -> Self {
        self.row_height = Some(h);
        self
    }
}

impl Default for TableRowProperty {
    fn default() -> Self {
        TableRowProperty {
            grid_after: None,
            width_after: None,
            row_height: None,
        }
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
