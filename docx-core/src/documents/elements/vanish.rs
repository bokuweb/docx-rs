use serde::{Deserialize, Serialize, Serializer};
use std::io::Write;

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, Deserialize, PartialEq, Default)]
pub struct Vanish {}

impl Vanish {
    pub fn new() -> Vanish {
        Vanish {}
    }
}

impl BuildXML for Vanish {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream).vanish()?.into_inner()
    }
}

impl Serialize for Vanish {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bool(true)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_build() {
        let c = Vanish::new();
        let b = c.build();
        assert_eq!(str::from_utf8(&b).unwrap(), r#"<w:vanish />"#);
    }
}
