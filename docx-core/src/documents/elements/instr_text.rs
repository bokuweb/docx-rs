use serde::Serialize;

use crate::documents::*;
use crate::xml_builder::*;

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct InstrText {
    pub val: String,
}

impl InstrText {
    pub fn new(i: impl Into<String>) -> Self {
        Self { val: i.into() }
    }
}

impl BuildXML for InstrText {
    fn build(&self) -> Vec<u8> {
        XMLBuilder::new()
            .open_instr_text()
            .plain_text(&self.val)
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
    fn test_toc_instr() {
        let b = InstrText::new(r#"ToC \o "1-3""#).build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:instrText>ToC \o "1-3"</w:instrText>"#
        );
    }
}
