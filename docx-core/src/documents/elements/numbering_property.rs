use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;

use super::{IndentLevel, NumberingId};
use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, PartialEq)]
pub struct NumberingProperty {
    pub id: Option<NumberingId>,
    pub level: Option<IndentLevel>,
}

impl Default for NumberingProperty {
    fn default() -> Self {
        NumberingProperty {
            id: None,
            level: None,
        }
    }
}

impl NumberingProperty {
    pub fn new() -> NumberingProperty {
        Default::default()
    }

    pub fn id(mut self, id: NumberingId) -> NumberingProperty {
        self.id = Some(id);
        self
    }

    pub fn add_num(mut self, id: NumberingId, level: IndentLevel) -> NumberingProperty {
        self.id = Some(id);
        self.level = Some(level);
        self
    }
}

impl BuildXML for NumberingProperty {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.open_numbering_property()
            .add_optional_child(&self.id)
            .add_optional_child(&self.level)
            .close()
            .build()
    }
}

impl Serialize for NumberingProperty {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut t = serializer.serialize_struct("NumberProperty", 2)?;
        let mut id: Option<usize> = None;
        if let Some(n) = &self.id {
            id = Some(n.id);
        }
        t.serialize_field("id", &id)?;

        let mut level: Option<usize> = None;
        if let Some(n) = &self.level {
            level = Some(n.val);
        }
        t.serialize_field("level", &level)?;
        t.end()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_num_property() {
        let c = NumberingProperty::new().add_num(NumberingId::new(0), IndentLevel::new(3));
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:numPr><w:numId w:val="0" /><w:ilvl w:val="3" /></w:numPr>"#
        );
    }

    #[test]
    fn test_empty_num_property() {
        let c = NumberingProperty::new();
        let b = c.build();
        assert_eq!(str::from_utf8(&b).unwrap(), r#"<w:numPr />"#);
    }

    #[test]
    fn test_num_property_json() {
        let c = NumberingProperty::new().add_num(NumberingId::new(0), IndentLevel::new(3));
        assert_eq!(serde_json::to_string(&c).unwrap(), r#"{"id":0,"level":3}"#);
    }

    #[test]
    fn test_empty_num_property_json() {
        let c = NumberingProperty::new();
        assert_eq!(
            serde_json::to_string(&c).unwrap(),
            r#"{"id":null,"level":null}"#
        );
    }
}
