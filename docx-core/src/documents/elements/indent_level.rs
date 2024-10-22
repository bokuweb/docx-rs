use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, PartialEq)]
pub struct IndentLevel {
    pub val: usize,
}

impl IndentLevel {
    pub fn new(val: usize) -> IndentLevel {
        IndentLevel { val }
    }
}

impl BuildXML for IndentLevel {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new(Vec::new());
        b.indent_level(self.val).into_inner()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_indent_level() {
        let c = IndentLevel::new(20);
        let b = c.build();
        assert_eq!(str::from_utf8(&b).unwrap(), r#"<w:ilvl w:val="20" />"#);
    }
}
