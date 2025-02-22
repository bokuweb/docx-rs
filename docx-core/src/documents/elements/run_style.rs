use serde::{Serialize, Serializer};
use std::io::Write;

use crate::documents::BuildXML;
use crate::escape::escape;
use crate::xml_builder::*;

#[derive(Debug, Clone, PartialEq)]
pub struct RunStyle {
    pub val: String,
}

impl Default for RunStyle {
    fn default() -> Self {
        RunStyle {
            val: "Normal".to_owned(),
        }
    }
}

impl RunStyle {
    pub fn new(val: impl Into<String>) -> RunStyle {
        RunStyle {
            val: escape(&val.into()),
        }
    }
}

impl BuildXML for RunStyle {
    fn build_to<W: Write>(
        &self,
        stream: xml::writer::EventWriter<W>,
    ) -> xml::writer::Result<xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream).run_style(&self.val)?.into_inner()
    }
}

impl Serialize for RunStyle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.val)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_r_style() {
        let c = RunStyle::new("Heading");
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:rStyle w:val="Heading" />"#
        );
    }
}
