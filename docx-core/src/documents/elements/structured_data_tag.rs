use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;

use super::*;
use crate::documents::BuildXML;
// use crate::types::*;
use crate::xml_builder::*;

#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StructuredDataTag {
    pub children: Vec<StructuredDataTagChild>,
    pub property: StructuredDataTagProperty,
    pub has_numbering: bool,
}

impl Default for StructuredDataTag {
    fn default() -> Self {
        Self {
            children: Vec::new(),
            property: StructuredDataTagProperty::new(),
            has_numbering: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum StructuredDataTagChild {
    Run(Box<Run>),
    Paragraph(Box<Paragraph>),
    Table(Box<Table>),
    BookmarkStart(BookmarkStart),
    BookmarkEnd(BookmarkEnd),
    CommentStart(Box<CommentRangeStart>),
    CommentEnd(CommentRangeEnd),
    StructuredDataTag(Box<StructuredDataTag>),
}

impl BuildXML for StructuredDataTagChild {
    fn build(&self) -> Vec<u8> {
        match self {
            StructuredDataTagChild::Run(v) => v.build(),
            StructuredDataTagChild::Paragraph(v) => v.build(),
            StructuredDataTagChild::Table(v) => v.build(),
            StructuredDataTagChild::BookmarkStart(v) => v.build(),
            StructuredDataTagChild::BookmarkEnd(v) => v.build(),
            StructuredDataTagChild::CommentStart(v) => v.build(),
            StructuredDataTagChild::CommentEnd(v) => v.build(),
            StructuredDataTagChild::StructuredDataTag(v) => v.build(),
        }
    }
}

impl Serialize for StructuredDataTagChild {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            StructuredDataTagChild::Run(ref r) => {
                let mut t = serializer.serialize_struct("Run", 2)?;
                t.serialize_field("type", "run")?;
                t.serialize_field("data", r)?;
                t.end()
            }
            StructuredDataTagChild::Paragraph(ref r) => {
                let mut t = serializer.serialize_struct("Paragraph", 2)?;
                t.serialize_field("type", "paragraph")?;
                t.serialize_field("data", r)?;
                t.end()
            }
            StructuredDataTagChild::Table(ref r) => {
                let mut t = serializer.serialize_struct("Table", 2)?;
                t.serialize_field("type", "table")?;
                t.serialize_field("data", r)?;
                t.end()
            }
            StructuredDataTagChild::BookmarkStart(ref c) => {
                let mut t = serializer.serialize_struct("BookmarkStart", 2)?;
                t.serialize_field("type", "bookmarkStart")?;
                t.serialize_field("data", c)?;
                t.end()
            }
            StructuredDataTagChild::BookmarkEnd(ref c) => {
                let mut t = serializer.serialize_struct("BookmarkEnd", 2)?;
                t.serialize_field("type", "bookmarkEnd")?;
                t.serialize_field("data", c)?;
                t.end()
            }
            StructuredDataTagChild::CommentStart(ref r) => {
                let mut t = serializer.serialize_struct("CommentRangeStart", 2)?;
                t.serialize_field("type", "commentRangeStart")?;
                t.serialize_field("data", r)?;
                t.end()
            }
            StructuredDataTagChild::CommentEnd(ref r) => {
                let mut t = serializer.serialize_struct("CommentRangeEnd", 2)?;
                t.serialize_field("type", "commentRangeEnd")?;
                t.serialize_field("data", r)?;
                t.end()
            }
            StructuredDataTagChild::StructuredDataTag(ref r) => {
                let mut t = serializer.serialize_struct("StructuredDataTag", 2)?;
                t.serialize_field("type", "structuredDataTag")?;
                t.serialize_field("data", r)?;
                t.end()
            }
        }
    }
}

impl StructuredDataTag {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_run(mut self, run: Run) -> Self {
        self.children
            .push(StructuredDataTagChild::Run(Box::new(run)));
        self
    }

    pub fn add_paragraph(mut self, p: Paragraph) -> Self {
        if p.has_numbering {
            self.has_numbering = true
        }
        self.children
            .push(StructuredDataTagChild::Paragraph(Box::new(p)));
        self
    }

    pub fn add_table(mut self, t: Table) -> Self {
        if t.has_numbering {
            self.has_numbering = true
        }
        self.children
            .push(StructuredDataTagChild::Table(Box::new(t)));
        self
    }

    pub fn data_binding(mut self, d: DataBinding) -> Self {
        self.property = self.property.data_binding(d);
        self
    }

    pub fn alias(mut self, v: impl Into<String>) -> Self {
        self.property = self.property.alias(v);
        self
    }

    fn inner_build(&self) -> Vec<u8> {
        XMLBuilder::new()
            .open_structured_tag()
            .add_child(&self.property)
            .open_structured_tag_content()
            .add_children(&self.children)
            .close()
            .close()
            .build()
    }
}

impl BuildXML for StructuredDataTag {
    fn build(&self) -> Vec<u8> {
        self.inner_build()
    }
}

impl BuildXML for Box<StructuredDataTag> {
    fn build(&self) -> Vec<u8> {
        self.inner_build()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_sdt() {
        let b = StructuredDataTag::new()
            .data_binding(DataBinding::new().xpath("root/hello"))
            .add_run(Run::new().add_text("Hello"))
            .build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:sdt><w:sdtPr><w:rPr /><w:dataBinding w:xpath="root/hello" /></w:sdtPr><w:sdtContent><w:r><w:rPr /><w:t xml:space="preserve">Hello</w:t></w:r></w:sdtContent>
</w:sdt>"#
        );
    }
}
