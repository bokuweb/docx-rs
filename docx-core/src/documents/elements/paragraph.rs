use super::{Delete, Insert, ParagraphProperty, Run};
use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;

#[derive(Debug, Clone)]
pub struct Paragraph<'a> {
    children: Vec<ParagraphChild<'a>>,
    property: ParagraphProperty,
    attrs: Vec<(String, String)>,
}

impl<'a> Default for Paragraph<'a> {
    fn default() -> Self {
        Self {
            children: Vec::new(),
            property: ParagraphProperty::new(),
            attrs: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ParagraphChild<'a> {
    Run(Run),
    Insert(Insert<'a>),
    Delete(Delete<'a>),
}

impl<'a> BuildXML for ParagraphChild<'a> {
    fn build(&self) -> Vec<u8> {
        match self {
            ParagraphChild::Run(v) => v.build(),
            ParagraphChild::Insert(v) => v.build(),
            ParagraphChild::Delete(v) => v.build(),
        }
    }
}

impl<'a> Paragraph<'a> {
    pub fn new() -> Paragraph<'a> {
        Default::default()
    }

    pub fn add_run(mut self, run: Run) -> Paragraph<'a> {
        self.children.push(ParagraphChild::Run(run));
        self
    }

    pub fn add_insert(mut self, insert: Insert<'a>) -> Paragraph<'a> {
        self.children.push(ParagraphChild::Insert(insert));
        self
    }

    pub fn add_delete(mut self, delete: Delete<'a>) -> Paragraph<'a> {
        self.children.push(ParagraphChild::Delete(delete));
        self
    }

    pub fn add_attr(mut self, key: impl Into<String>, val: impl Into<String>) -> Paragraph<'a> {
        self.attrs.push((key.into(), val.into()));
        self
    }

    pub fn align(mut self, alignment_type: AlignmentType) -> Paragraph<'a> {
        self.property = self.property.align(alignment_type);
        self
    }

    // pub fn size(mut self, size: usize) -> Paragraph<'a> {
    //     self.children = self.children.into_iter().map(|r| r.size(size)).collect();
    //     self
    // }

    pub fn style(mut self, style_id: &str) -> Paragraph<'a> {
        self.property = self.property.style(style_id);
        self
    }

    pub fn indent(
        mut self,
        left: usize,
        special_indent: Option<SpecialIndentType>,
    ) -> Paragraph<'a> {
        self.property = self.property.indent(left, special_indent);
        self
    }
}

impl<'a> BuildXML for Paragraph<'a> {
    fn build(&self) -> Vec<u8> {
        XMLBuilder::new()
            .open_paragraph(&self.attrs)
            .add_child(&self.property)
            .add_children(&self.children)
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
        let b = Paragraph::new()
            .add_run(Run::new().add_text("Hello"))
            .build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:p><w:pPr><w:pStyle w:val="Normal" /><w:rPr /></w:pPr><w:r><w:rPr /><w:t xml:space="preserve">Hello</w:t></w:r></w:p>"#
        );
    }

    #[test]
    fn test_custom_attr() {
        let b = Paragraph::new()
            .add_run(Run::new().add_text("Hello"))
            .add_attr("customId", "abcd-1234-567890")
            .build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:p customId="abcd-1234-567890"><w:pPr><w:pStyle w:val="Normal" /><w:rPr /></w:pPr><w:r><w:rPr /><w:t xml:space="preserve">Hello</w:t></w:r></w:p>"#
        );
    }
}
