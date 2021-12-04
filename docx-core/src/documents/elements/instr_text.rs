use serde::Serialize;

use crate::documents::*;
use crate::types::*;
use crate::xml_builder::*;

#[derive(Serialize, Debug, Clone, PartialEq)]
pub enum InstrText {
    ToC(ToC),
    Unsupported,
}

// https://c-rex.net/projects/samples/ooxml/e1/Part4/OOXML_P4_DOCX_TOCTOC_topic_ID0ELZO1.html
#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct ToC {}

impl BuildXML for InstrText {
    fn build(&self) -> Vec<u8> {
        XMLBuilder::new()
            .field_character(
                &format!("{}", self.field_char_type),
                &format!("{}", &self.dirty),
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
    fn test_field_character() {
        let b = InstrText::new(FieldCharType::Begin).dirty().build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:fldChar w:type="begin" w:dirty="true" />"#
        );
    }
}
