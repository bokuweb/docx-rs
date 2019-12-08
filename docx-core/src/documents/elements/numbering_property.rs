use super::{IndentLevel, NumberingId};
use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone)]
pub struct NumberingProperty {
    id: NumberingId,
    level: IndentLevel,
}

impl NumberingProperty {
    pub fn new(id: NumberingId, level: IndentLevel) -> NumberingProperty {
        Self { id, level }
    }
}

impl BuildXML for NumberingProperty {
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
        let c = NumberingProperty::new(NumberingId::new(0), IndentLevel::new(3));
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:numPr><w:numId w:val="0" /><w:ilvl w:val="3" /></w:numPr>"#
        );
    }
}
