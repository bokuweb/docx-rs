use crate::documents::BuildXML;
use crate::xml_builder::*;
use serde::{Deserialize, Serialize, Serializer};
use std::io::Write;

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct SzCs {
    pub val: usize,
}

impl SzCs {
    pub fn new(val: usize) -> SzCs {
        SzCs { val }
    }
}

impl BuildXML for SzCs {
    fn build_to<W: Write>(
        &self,
        stream: xml::writer::EventWriter<W>,
    ) -> xml::writer::Result<xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream).sz_cs(self.val)?.into_inner()
    }
}

impl Serialize for SzCs {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u32(self.val as u32)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_sz_cs() {
        let c = SzCs::new(20);
        let b = c.build();
        assert_eq!(str::from_utf8(&b).unwrap(), r#"<w:szCs w:val="20" />"#);
    }
}
