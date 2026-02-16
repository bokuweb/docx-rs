use super::*;
// use crate::create_header_rid;
use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;
use crate::Footer;
use crate::{delegate_to_field, Header};
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
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
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
    pub(crate) property: SectionProperty,
    pub(crate) children: Vec<SectionChild>,
    pub(crate) has_numbering: bool,
    pub(crate) temp_header: Option<Header>,
    pub(crate) temp_first_header: Option<Header>,
    pub(crate) temp_even_header: Option<Header>,
    pub(crate) temp_footer: Option<Footer>,
    pub(crate) temp_first_footer: Option<Footer>,
    pub(crate) temp_even_footer: Option<Footer>,
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
        // header(h: Header, rid: &str) -> Self,
        // first_header(h: Header, rid: &str) -> Self,
        // first_header_without_title_pg(h: Header, rid: &str) -> Self,
        // even_header(h: Header, rid: &str) -> Self,
        // footer(h: Footer, rid: &str) -> Self,
        // first_footer(h: Footer, rid: &str) -> Self,
        // first_footer_without_title_pg(h: Footer, rid: &str) -> Self,
        // even_footer(h: Footer, rid: &str) -> Self,
        page_num_type(h: PageNumType) -> Self,
    }

    pub fn add_paragraph(mut self, p: Paragraph) -> Self {
        if p.has_numbering {
            self.has_numbering = true
        }
        self.children.push(SectionChild::Paragraph(Box::new(p)));
        self
    }

    pub fn add_table(mut self, t: Table) -> Self {
        if t.has_numbering {
            self.has_numbering = true
        }
        self.children.push(SectionChild::Table(Box::new(t)));
        self
    }

    pub fn add_bookmark_start(mut self, id: usize, name: impl Into<String>) -> Self {
        self.children
            .push(SectionChild::BookmarkStart(BookmarkStart::new(id, name)));
        self
    }

    pub fn add_bookmark_end(mut self, id: usize) -> Self {
        self.children
            .push(SectionChild::BookmarkEnd(BookmarkEnd::new(id)));
        self
    }

    pub fn add_comment_start(mut self, comment: Comment) -> Self {
        self.children.push(SectionChild::CommentStart(Box::new(
            CommentRangeStart::new(comment),
        )));
        self
    }

    pub fn add_comment_end(mut self, id: usize) -> Self {
        self.children
            .push(SectionChild::CommentEnd(CommentRangeEnd::new(id)));
        self
    }

    pub fn header(mut self, header: Header) -> Self {
        self.temp_header = Some(header);
        self
    }

    pub fn first_header(mut self, header: Header) -> Self {
        self.temp_first_header = Some(header);
        self
    }

    pub fn even_header(mut self, header: Header) -> Self {
        self.temp_even_header = Some(header);
        self
    }

    pub fn footer(mut self, footer: Footer) -> Self {
        self.temp_footer = Some(footer);
        self
    }

    pub fn first_footer(mut self, footer: Footer) -> Self {
        self.temp_first_footer = Some(footer);
        self
    }

    pub fn even_footer(mut self, footer: Footer) -> Self {
        self.temp_even_footer = Some(footer);
        self
    }
}

impl Default for Section {
    fn default() -> Self {
        Self {
            property: SectionProperty::new(),
            children: vec![],
            has_numbering: false,
            temp_header: None,
            temp_first_header: None,
            temp_even_header: None,
            temp_footer: None,
            temp_first_footer: None,
            temp_even_footer: None,
        }
    }
}

impl BuildXML for Section {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
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

    #[test]
    fn test_section_with_paragraph() {
        let c =
            Section::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Hello")));
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:p w14:paraId="12345678"><w:pPr><w:rPr /></w:pPr><w:r><w:rPr /><w:t xml:space="preserve">Hello</w:t></w:r></w:p><w:p w14:paraId="12345678"><w:pPr><w:sectPr><w:pgSz w:w="11906" w:h="16838" /><w:pgMar w:top="1985" w:right="1701" w:bottom="1701" w:left="1701" w:header="851" w:footer="992" w:gutter="0" /><w:cols w:space="425" w:num="1" /></w:sectPr></w:pPr></w:p>"#
        );
    }
}
