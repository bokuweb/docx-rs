use super::Font;
use crate::documents::BuildXML;
use crate::types::FontPitchType;
use crate::xml_builder::*;

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FontTable {}

impl FontTable {
    pub fn new() -> FontTable {
        Default::default()
    }
}

impl Default for FontTable {
    fn default() -> Self {
        Self {}
    }
}

impl BuildXML for FontTable {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        let times = Font::new("Times New Roman", "00", "roman", FontPitchType::Variable);
        let symbol = Font::new("Symbol", "02", "roman", FontPitchType::Variable);
        let arial = Font::new("Arial", "00", "swiss", FontPitchType::Variable);
        b.declaration(Some(true))
            .open_fonts()
            .add_child(&times)
            .add_child(&symbol)
            .add_child(&arial)
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
    fn test_settings() {
        let c = FontTable::new();
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:fonts xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"><w:font w:name="Times New Roman">
  <w:charset w:val="00" />
  <w:family w:val="roman" />
  <w:pitch w:val="variable" />
</w:font><w:font w:name="Symbol">
  <w:charset w:val="02" />
  <w:family w:val="roman" />
  <w:pitch w:val="variable" />
</w:font><w:font w:name="Arial">
  <w:charset w:val="00" />
  <w:family w:val="swiss" />
  <w:pitch w:val="variable" />
</w:font></w:fonts>"#
        );
    }
}
