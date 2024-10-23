use serde::Serialize;
use std::io::Write;

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
    fn build_to<W: Write>(
        &self,
        stream: xml::writer::EventWriter<W>,
    ) -> xml::writer::Result<xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_paragraph_property_default()?
            .add_child(&self.paragraph_property)?
            .close()?
            .into_inner()
    }
}

mod tests {

    #[allow(unused_imports)]
    use super::*;

    #[cfg(test)]
    use pretty_assertions::assert_eq;

    #[test]
    fn test_build() {
        let c = ParagraphPropertyDefault::new();
        let b = c.build();
        assert_eq!(
            std::str::from_utf8(&b).unwrap(),
            r#"<w:pPrDefault><w:pPr><w:rPr /></w:pPr></w:pPrDefault>"#
        );
    }
}
