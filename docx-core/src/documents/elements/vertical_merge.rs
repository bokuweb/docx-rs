use serde::{Serialize, Serializer};
use std::io::Write;

use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;

#[derive(Debug, Clone, PartialEq)]
pub struct VMerge {
    val: VMergeType,
}

impl VMerge {
    pub fn new(v: VMergeType) -> VMerge {
        VMerge { val: v }
    }
}

impl BuildXML for VMerge {
    fn build_to<W: Write>(
        &self,
        stream: xml::writer::EventWriter<W>,
    ) -> xml::writer::Result<xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .vertical_merge(&self.val.to_string())?
            .into_inner()
    }
}

impl Serialize for VMerge {
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
        let b = VMerge::new(VMergeType::Continue).build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:vMerge w:val="continue" />"#
        );
    }
}
