use super::*;
use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;
use crate::{delegate_getters_to_field, delegate_to_field, Footer, Header};
use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;
use std::io::Write;

#[derive(Debug, Clone, PartialEq)]
pub enum SectionChild {
    Paragraph(Box<Paragraph>),
    Table(Box<Table>),
    BookmarkStart(BookmarkStart),
    BookmarkEnd(BookmarkEnd),
    CommentStart(Box<CommentRangeStart>),
    CommentEnd(CommentRangeEnd),
    StructuredDataTag(Box<StructuredDataTag>),
    TableOfContents(Box<TableOfContents>),
}

impl Serialize for SectionChild {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            SectionChild::Paragraph(ref p) => {
                let mut t = serializer.serialize_struct("Paragraph", 2)?;
                t.serialize_field("type", "paragraph")?;
                t.serialize_field("data", p)?;
                t.end()
            }
            SectionChild::Table(ref c) => {
                let mut t = serializer.serialize_struct("Table", 2)?;
                t.serialize_field("type", "table")?;
                t.serialize_field("data", c)?;
                t.end()
            }
            SectionChild::BookmarkStart(ref c) => {
                let mut t = serializer.serialize_struct("BookmarkStart", 2)?;
                t.serialize_field("type", "bookmarkStart")?;
                t.serialize_field("data", c)?;
                t.end()
            }
            SectionChild::BookmarkEnd(ref c) => {
                let mut t = serializer.serialize_struct("BookmarkEnd", 2)?;
                t.serialize_field("type", "bookmarkEnd")?;
                t.serialize_field("data", c)?;
                t.end()
            }
            SectionChild::CommentStart(ref r) => {
                let mut t = serializer.serialize_struct("CommentRangeStart", 2)?;
                t.serialize_field("type", "commentRangeStart")?;
                t.serialize_field("data", r)?;
                t.end()
            }
            SectionChild::CommentEnd(ref r) => {
                let mut t = serializer.serialize_struct("CommentRangeEnd", 2)?;
                t.serialize_field("type", "commentRangeEnd")?;
                t.serialize_field("data", r)?;
                t.end()
            }
            SectionChild::StructuredDataTag(ref r) => {
                let mut t = serializer.serialize_struct("StructuredDataTag", 2)?;
                t.serialize_field("type", "structuredDataTag")?;
                t.serialize_field("data", r)?;
                t.end()
            }
            SectionChild::TableOfContents(ref r) => {
                let mut t = serializer.serialize_struct("TableOfContents", 2)?;
                t.serialize_field("type", "tableOfContents")?;
                t.serialize_field("data", r)?;
                t.end()
            }
        }
    }
}

impl BuildXML for SectionChild {
    fn build_to<W: Write>(
        &self,
        stream: xml::writer::EventWriter<W>,
    ) -> xml::writer::Result<xml::writer::EventWriter<W>> {
        match self {
            SectionChild::Paragraph(v) => v.build_to(stream),
            SectionChild::Table(v) => v.build_to(stream),
            SectionChild::BookmarkStart(v) => v.build_to(stream),
            SectionChild::BookmarkEnd(v) => v.build_to(stream),
            SectionChild::CommentStart(v) => v.build_to(stream),
            SectionChild::CommentEnd(v) => v.build_to(stream),
            SectionChild::StructuredDataTag(v) => v.build_to(stream),
            SectionChild::TableOfContents(v) => v.build_to(stream),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Section {
    property: SectionProperty,
    children: Vec<SectionChild>,
}

impl Section {
    pub fn new() -> Section {
        Default::default()
    }

    delegate_to_field! {
        property =>
        page_size(size: PageSize) -> Self,
        page_margin(margin: PageMargin) -> Self,
        page_orient(o: PageOrientationType) -> Self,
        doc_grid(doc_grid: DocGrid) -> Self,
        text_direction(direction: String) -> Self,
        title_pg() -> Self,
        header(h: Header, rid: &str) -> Self,
        first_header(h: Header, rid: &str) -> Self,
        first_header_without_title_pg(h: Header, rid: &str) -> Self,
        even_header(h: Header, rid: &str) -> Self,
        footer(h: Footer, rid: &str) -> Self,
        first_footer(h: Footer, rid: &str) -> Self,
        first_footer_without_title_pg(h: Footer, rid: &str) -> Self,
        even_footer(h: Footer, rid: &str) -> Self,
        page_num_type(h: PageNumType) -> Self,
    }

    delegate_getters_to_field! {
        property =>
        get_headers() -> Vec<&Header>,
        get_footers() -> Vec<&Footer>,
    }
}

impl Default for Section {
    fn default() -> Self {
        Self {
            property: SectionProperty::new(),
            children: vec![],
        }
    }
}

impl BuildXML for Section {
    fn build_to<W: Write>(
        &self,
        stream: xml::writer::EventWriter<W>,
    ) -> xml::writer::Result<xml::writer::EventWriter<W>> {
        let id = crate::generate_para_id();
        XMLBuilder::from(stream)
            .add_children(&self.children)?
            .open_paragraph(&id)?
            .open_paragraph_property()?
            .add_child(&self.property)?
            .close()?
            .close()?
            .into_inner()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_section_property_default() {
        let c = Section::new();
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:p w14:paraId="12345678"><w:pPr><w:sectPr><w:pgSz w:w="11906" w:h="16838" /><w:pgMar w:top="1985" w:right="1701" w:bottom="1701" w:left="1701" w:header="851" w:footer="992" w:gutter="0" /><w:cols w:space="425" w:num="1" /></w:sectPr></w:pPr></w:p>"#
        );
    }
}
