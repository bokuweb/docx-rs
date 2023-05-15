use serde::Serialize;

use crate::documents::*;
use crate::types::*;
use crate::xml_builder::*;

#[derive(Serialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "wasm", derive(ts_rs::TS))]
#[cfg_attr(feature = "wasm", ts(export))]
#[serde(rename_all = "camelCase")]
pub struct FieldChar {
    pub field_char_type: FieldCharType,
    pub dirty: bool,
}

impl FieldChar {
    pub fn new(t: FieldCharType) -> Self {
        Self {
            field_char_type: t,
            dirty: false,
        }
    }

    pub fn dirty(mut self) -> Self {
        self.dirty = true;
        self
    }
}

impl BuildXML for FieldChar {
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
        let b = FieldChar::new(FieldCharType::Begin).dirty().build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:fldChar w:fldCharType="begin" w:dirty="true" />"#
        );
    }
}
