use serde::{Serialize, Serializer};
use std::io::Write;

use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;

#[derive(Debug, Clone, PartialEq)]
pub struct VAlign {
    val: VAlignType,
}

impl VAlign {
    pub fn new(v: VAlignType) -> VAlign {
        VAlign { val: v }
    }
}

impl BuildXML for VAlign {
    fn build_to<W: Write>(
        &self,
        stream: xml::writer::EventWriter<W>,
    ) -> xml::writer::Result<xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .vertical_align(&self.val.to_string())?
            .into_inner()
    }
}

impl Serialize for VAlign {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{}", &self.val))
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
        let b = VAlign::new(VAlignType::Center).build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:vAlign w:val="center" />"#
        );
    }
}
