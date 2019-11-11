use super::{ParagraphProperty, Run, RunProperty, Text};
use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;

#[derive(Debug)]
pub struct Paragraph {
    runs: Vec<Run>,
    property: ParagraphProperty,
}

impl Default for Paragraph {
    fn default() -> Self {
        Self {
            runs: Vec::new(),
            property: ParagraphProperty::new(),
        }
    }
}

impl Paragraph {
    pub fn new() -> Paragraph {
        Default::default()
    }

    pub fn add_run(mut self, run: Run) -> Paragraph {
        self.runs.push(run);
        self
    }

    pub fn align(mut self, alignment_type: AlignmentType) -> Paragraph {
        self.property = self.property.align(alignment_type);
        self
    }
}

impl BuildXML for Paragraph {
    fn build(&self) -> Vec<u8> {
        XMLBuilder::new()
            .open_paragraph()
            .add_child(&self.property)
            .add_children(&self.runs)
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
    fn test_paragraph() {
        let b = Paragraph::new().add_run(Run::new("Hello")).build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:p><w:r><w:rPr /><w:t>Hello</w:t></w:r></w:p>"#
        );
    }
}
