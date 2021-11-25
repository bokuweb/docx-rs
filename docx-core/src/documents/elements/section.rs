use super::*;
use crate::documents::BuildXML;
use crate::xml_builder::*;

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Section {
    property: SectionProperty,
}

impl Section {
    pub fn new() -> Section {
        Default::default()
    }
}

impl Default for Section {
    fn default() -> Self {
        Self {
            property: SectionProperty::new(),
        }
    }
}

impl BuildXML for Section {
    fn build(&self) -> Vec<u8> {
        let id = crate::generate_para_id();

        XMLBuilder::new()
            .open_paragraph(&id)
            .open_paragraph_property()
            .add_child(&self.property)
            .close()
            .close()
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
    fn test_section_property_default() {
        let c = Section::new();
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:p w14:paraId="12345678">
  <w:pPr><w:sectPr><w:pgSz w:w="11906" w:h="16838" /><w:pgMar w:top="1985" w:right="1701" w:bottom="1701" w:left="1701" w:header="851" w:footer="992" w:gutter="0" /><w:cols w:space="425" /><w:docGrid w:type="lines" w:linePitch="360" /></w:sectPr></w:pPr>
</w:p>"#
        );
    }
}
