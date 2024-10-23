use crate::documents::BuildXML;
use crate::xml_builder::*;
use std::io::Write;

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
    fn build_to<W: Write>(
        &self,
        stream: xml::writer::EventWriter<W>,
    ) -> xml::writer::Result<xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .indent_level(self.val)?
            .into_inner()
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
