use serde::{Serialize, Serializer};
use std::io::Write;

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Next {
    val: String,
}

impl Next {
    pub fn new(val: impl Into<String>) -> Next {
        Next { val: val.into() }
    }
}

impl Serialize for Next {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.val)
    }
}

impl BuildXML for Next {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream).next(&self.val)?.into_inner()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_next() {
        let c = Next::new("Normal");
        let b = c.build();
        assert_eq!(str::from_utf8(&b).unwrap(), r#"<w:next w:val="Normal" />"#);
    }
}
