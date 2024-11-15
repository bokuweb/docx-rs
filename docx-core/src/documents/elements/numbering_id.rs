use crate::documents::BuildXML;
use crate::xml_builder::*;
use std::io::Write;

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct NumberingId {
    pub id: usize,
}

impl NumberingId {
    pub fn new(id: usize) -> NumberingId {
        NumberingId { id }
    }
}

impl BuildXML for NumberingId {
    fn build_to<W: Write>(
        &self,
        stream: xml::writer::EventWriter<W>,
    ) -> xml::writer::Result<xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream).num_id(self.id)?.into_inner()
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
        let c = NumberingId::new(0);
        let b = c.build();
        assert_eq!(str::from_utf8(&b).unwrap(), r#"<w:numId w:val="0" />"#);
    }
}
