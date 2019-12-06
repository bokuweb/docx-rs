use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone)]
pub struct NumberFormat<'a> {
    val: &'a str,
}

impl<'a> NumberFormat<'a> {
    pub fn new(val: &'a str) -> Self {
        Self { val }
    }
}

impl<'a> BuildXML for NumberFormat<'a> {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.number_format(self.val).build()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_start() {
        let c = NumberFormat::new("decimal");
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:numFmt w:val="decimal" />"#
        );
    }
}
