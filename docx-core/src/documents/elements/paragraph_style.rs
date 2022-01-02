use serde::{Serialize, Serializer};

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, PartialEq)]
pub struct ParagraphStyle {
    pub val: String,
}

impl Default for ParagraphStyle {
    fn default() -> Self {
        ParagraphStyle {
            val: "Normal".to_owned(),
        }
    }
}

// 17.9.23
// pStyle (Paragraph Style's Associated Numbering Level)
// This element specifies the name of a paragraph style which shall automatically this numbering level when
// applied to the contents of the document. When a paragraph style is defined to include a numbering definition,
// any numbering level defined by the numPr element (§17.3.1.19) shall be ignored, and instead this element shall
// specify the numbering level associated with that paragraph style.
impl ParagraphStyle {
    pub fn new(val: Option<impl Into<String>>) -> ParagraphStyle {
        if let Some(v) = val {
            ParagraphStyle { val: v.into() }
        } else {
            Default::default()
        }
    }
}

impl BuildXML for ParagraphStyle {
    fn build(&self) -> Vec<u8> {
        XMLBuilder::new().paragraph_style(&self.val).build()
    }
}

impl Serialize for ParagraphStyle {
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
    fn test_p_style() {
        let c = ParagraphStyle::new(Some("Heading"));
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:pStyle w:val="Heading" />"#
        );
    }
}
