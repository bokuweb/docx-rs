use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone)]
pub struct NumberingId<'a> {
    id: &'a str,
}

impl<'a> NumberingId<'a> {
    pub fn new(id: &'a str) -> NumberingId<'a> {
        NumberingId { id }
    }
}

impl<'a> BuildXML for NumberingId<'a> {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.num_id(self.id).build()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_num_id() {
        let c = NumberingId::new("abc");
        let b = c.build();
        assert_eq!(str::from_utf8(&b).unwrap(), r#"<w:numId w:val="abc" />"#);
    }
}
