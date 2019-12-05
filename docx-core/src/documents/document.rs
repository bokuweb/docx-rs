use super::{Paragraph, Table};
use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug)]
pub struct Document<'a> {
    pub(crate) children: Vec<DocumentChild<'a>>,
}

#[derive(Debug, Clone)]
pub enum DocumentChild<'a> {
    Paragraph(Paragraph<'a>),
    Table(Table<'a>),
}

impl<'a> Document<'a> {
    pub fn new() -> Document<'a> {
        Default::default()
    }

    pub fn add_paragraph(mut self, p: Paragraph<'a>) -> Self {
        self.children.push(DocumentChild::Paragraph(p));
        self
    }

    pub fn add_table(mut self, t: Table<'a>) -> Self {
        self.children.push(DocumentChild::Table(t));
        self
    }
}

impl<'a> Default for Document<'a> {
    fn default() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl<'a> BuildXML for Document<'a> {
    fn build(&self) -> Vec<u8> {
        let mut b = XMLBuilder::new()
            .declaration(Some(true))
            .open_document()
            .open_body();
        for c in &self.children {
            match c {
                DocumentChild::Paragraph(p) => b = b.add_child(p),
                DocumentChild::Table(t) => b = b.add_child(t),
            }
        }
        b.close().close().build()
    }
}

#[cfg(test)]
mod tests {

    use super::super::Run;
    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_document() {
        let b = Document::new()
            .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Hello")))
            .build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:document xmlns:o="urn:schemas-microsoft-com:office:office" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:v="urn:schemas-microsoft-com:vml" xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:w10="urn:schemas-microsoft-com:office:word" xmlns:wp="http://schemas.openxmlformats.org/drawingml/2006/wordprocessingDrawing" xmlns:wps="http://schemas.microsoft.com/office/word/2010/wordprocessingShape" xmlns:wpg="http://schemas.microsoft.com/office/word/2010/wordprocessingGroup" xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006" xmlns:wp14="http://schemas.microsoft.com/office/word/2010/wordprocessingDrawing" xmlns:w14="http://schemas.microsoft.com/office/word/2010/wordml" mc:Ignorable="w14 wp14">
  <w:body><w:p><w:pPr><w:pStyle w:val="Normal" /><w:rPr /></w:pPr><w:r><w:rPr /><w:t xml:space="preserve">Hello</w:t></w:r></w:p></w:body>
</w:document>"#
        );
    }
}
