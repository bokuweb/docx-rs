use super::{Justification, ParagraphStyle, RunProperty, Sz, SzCs};
use crate::documents::BuildXML;
use crate::types::AlignmentType;
use crate::xml_builder::*;

#[derive(Debug)]
pub struct ParagraphProperty {
    alignment: Option<Justification>,
    run_property: RunProperty,
    style: ParagraphStyle,
}

impl Default for ParagraphProperty {
    fn default() -> Self {
        let s: Option<&str> = None;
        ParagraphProperty {
            alignment: None,
            run_property: RunProperty::new(),
            style: ParagraphStyle::new(s),
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
}

impl BuildXML for ParagraphProperty {
    fn build(&self) -> Vec<u8> {
        XMLBuilder::new()
            .open_paragraph_property()
            .add_optional_child(&self.alignment)
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
        assert_eq!(str::from_utf8(&b).unwrap(), r#"<w:pPr />"#);
    }

    #[test]
    fn test_alignment() {
        let c = ParagraphProperty::new();
        let b = c.align(AlignmentType::Right).build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:pPr><w:jc w:val="right" /></w:pPr>"#
        );
    }
}
