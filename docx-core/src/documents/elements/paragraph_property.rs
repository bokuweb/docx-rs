use super::{Indent, Justification, ParagraphStyle, RunProperty};
use crate::documents::BuildXML;
use crate::types::{AlignmentType, SpecialIndentType};
use crate::xml_builder::*;

#[derive(Debug)]
pub struct ParagraphProperty {
    run_property: RunProperty,
    style: ParagraphStyle,
    alignment: Option<Justification>,
    indent: Option<Indent>,
}

impl Default for ParagraphProperty {
    fn default() -> Self {
        let s: Option<&str> = None;
        ParagraphProperty {
            run_property: RunProperty::new(),
            style: ParagraphStyle::new(s),
            alignment: None,
            indent: None,
        }
    }
}

// 17.3.1.26
// pPr (Paragraph Properties)
// This element specifies a set of paragraph properties which shall be applied to the contents of the parent
// paragraph after all style/numbering/table properties have been applied to the text. These properties are defined
// as direct formatting, since they are directly applied to the paragraph and supersede any formatting from styles.
impl ParagraphProperty {
    pub fn new() -> ParagraphProperty {
        Default::default()
    }

    pub fn align(mut self, alignment_type: AlignmentType) -> ParagraphProperty {
        self.alignment = Some(Justification::new(alignment_type.to_string()));
        self
    }

    pub fn style(mut self, style_id: &str) -> ParagraphProperty {
        self.style = ParagraphStyle::new(Some(style_id));
        self
    }

    pub fn indent(
        mut self,
        left: usize,
        special_indent: Option<SpecialIndentType>,
    ) -> ParagraphProperty {
        self.indent = Some(Indent::new(left, special_indent));
        self
    }
}

impl BuildXML for ParagraphProperty {
    fn build(&self) -> Vec<u8> {
        XMLBuilder::new()
            .open_paragraph_property()
            .add_child(&self.style)
            .add_child(&self.run_property)
            .add_optional_child(&self.alignment)
            .add_optional_child(&self.indent)
            .close()
            .build()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_default() {
        let c = ParagraphProperty::new();
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:pPr><w:pStyle w:val="Normal" /><w:rPr /></w:pPr>"#
        );
    }

    #[test]
    fn test_alignment() {
        let c = ParagraphProperty::new();
        let b = c.align(AlignmentType::Right).build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:pPr><w:pStyle w:val="Normal" /><w:rPr /><w:jc w:val="right" /></w:pPr>"#
        );
    }

    #[test]
    fn test_indent() {
        let c = ParagraphProperty::new();
        let b = c.indent(20, None).build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:pPr><w:pStyle w:val="Normal" /><w:rPr /><w:ind w:left="20" /></w:pPr>"#
        );
    }
}
