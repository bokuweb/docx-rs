use super::*;
use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct TextBoxContent {
    pub children: Vec<TextBoxContentChild>,
    pub has_numbering: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TextBoxContentChild {
    Paragraph(Paragraph),
    Table(Table),
}

impl Serialize for TextBoxContentChild {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            TextBoxContentChild::Paragraph(ref p) => {
                let mut t = serializer.serialize_struct("Paragraph", 2)?;
                t.serialize_field("type", "paragraph")?;
                t.serialize_field("data", p)?;
                t.end()
            }
            TextBoxContentChild::Table(ref c) => {
                let mut t = serializer.serialize_struct("Table", 2)?;
                t.serialize_field("type", "table")?;
                t.serialize_field("data", c)?;
                t.end()
            }
        }
    }
}

impl TextBoxContent {
    pub fn new() -> TextBoxContent {
        Default::default()
    }

    pub fn add_paragraph(mut self, p: Paragraph) -> Self {
        if p.has_numbering {
            self.has_numbering = true
        }
        self.children.push(TextBoxContentChild::Paragraph(p));
        self
    }

    pub fn add_table(mut self, t: Table) -> Self {
        if t.has_numbering {
            self.has_numbering = true
        }
        self.children.push(TextBoxContentChild::Table(t));
        self
    }
}

impl Default for TextBoxContent {
    fn default() -> Self {
        TextBoxContent {
            children: vec![],
            has_numbering: false,
        }
    }
}

impl BuildXML for TextBoxContent {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        let mut b = b.open_text_box_content();
        for c in &self.children {
            match c {
                TextBoxContentChild::Paragraph(p) => b = b.add_child(p),
                TextBoxContentChild::Table(t) => b = b.add_child(t),
            }
        }
        b.close().build()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_text_box_content_build() {
        let b = TextBoxContent::new()
            .add_paragraph(Paragraph::new())
            .build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:txbxContent><w:p><w:pPr><w:pStyle w:val="Normal" /><w:rPr /></w:pPr></w:p></w:txbxContent>"#
        );
    }
}
