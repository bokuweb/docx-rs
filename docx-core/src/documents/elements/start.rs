use serde::{Serialize, Serializer};
use std::io::Write;

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Start {
    val: usize,
}

impl Start {
    pub fn new(val: usize) -> Start {
        Start { val }
    }
}

impl BuildXML for Start {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream).start(self.val)?.into_inner()
    }
}

impl Serialize for Start {
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
    fn test_start() {
        let c = Start::new(1);
        let b = c.build();
        assert_eq!(str::from_utf8(&b).unwrap(), r#"<w:start w:val="1" />"#);
    }
}
