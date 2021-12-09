use serde::Serialize;

use crate::documents::*;
use crate::xml_builder::*;

#[derive(Serialize, Debug, Clone, PartialEq, Default)]
pub struct DataBinding {
    pub xpath: Option<String>,
    pub prefix_mappings: Option<String>,
    pub store_item_id: Option<String>,
}

impl DataBinding {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn xpath(mut self, xpath: impl Into<String>) -> Self {
        self.xpath = Some(xpath.into());
        self
    }

    pub fn prefix_mappings(mut self, m: impl Into<String>) -> Self {
        self.prefix_mappings = Some(m.into());
        self
    }

    pub fn store_item_id(mut self, id: impl Into<String>) -> Self {
        self.store_item_id = Some(id.into());
        self
    }
}

impl BuildXML for DataBinding {
    fn build(&self) -> Vec<u8> {
        XMLBuilder::new()
            .data_binding(
                self.xpath.as_ref(),
                self.prefix_mappings.as_ref(),
                self.store_item_id.as_ref(),
            )
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
    fn test_delete_default() {
        let b = DataBinding::new().xpath("root/hello").build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:dataBinding w:xpath="root/hello" />"#
        );
    }
}
