use super::{IndentLevel, NumberingId};
use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone)]
pub struct NumberingProperty<'a> {
    id: NumberingId<'a>,
    level: IndentLevel,
}

impl<'a> NumberingProperty<'a> {
    pub fn new(id: NumberingId<'a>, level: IndentLevel) -> NumberingProperty {
        Self { id, level }
    }
}

impl<'a> BuildXML for NumberingProperty<'a> {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.open_numbering_property()
            .add_child(&self.id)
            .add_child(&self.level)
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
    fn test_num_property() {
        let c = NumberingProperty::new(NumberingId::new("abc"), IndentLevel::new(3));
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:numPr><w:numId w:val="abc" /><w:ilvl w:val="3" /></w:numPr>"#
        );
    }
}
