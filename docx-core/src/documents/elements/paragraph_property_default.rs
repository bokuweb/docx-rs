use serde::Serialize;

use super::*;
use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParagraphPropertyDefault {
    paragraph_property: ParagraphProperty,
}

impl ParagraphPropertyDefault {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn line_spacing(mut self, spacing: LineSpacing) -> Self {
        self.paragraph_property = self.paragraph_property.line_spacing(spacing);
        self
    }

    pub(crate) fn paragraph_property(mut self, p: ParagraphProperty) -> Self {
        self.paragraph_property = p;
        self
    }
}

impl Default for ParagraphPropertyDefault {
    fn default() -> Self {
        let paragraph_property = ParagraphProperty::new();
        ParagraphPropertyDefault { paragraph_property }
    }
}

impl BuildXML for ParagraphPropertyDefault {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.open_run_property_default()
            .add_child(&self.paragraph_property)
            .close()
            .build()
    }
}

mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;

    #[test]
    fn test_build() {
        let c = ParagraphPropertyDefault::new();
        let b = c.build();
        assert_eq!(
            std::str::from_utf8(&b).unwrap(),
            r#"<w:rPrDefault><w:pPr><w:rPr /></w:pPr></w:rPrDefault>"#
        );
    }
}
