use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;

use crate::types::*;
use crate::xml_builder::*;
use crate::{documents::*, escape};

#[derive(Debug, Clone, PartialEq)]
pub enum TocContent {
    Paragraph(Box<Paragraph>),
    Table(Box<Table>),
}

impl Serialize for TocContent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            TocContent::Paragraph(ref p) => {
                let mut t = serializer.serialize_struct("Paragraph", 2)?;
                t.serialize_field("type", "paragraph")?;
                t.serialize_field("data", p)?;
                t.end()
            }
            TocContent::Table(ref c) => {
                let mut t = serializer.serialize_struct("Table", 2)?;
                t.serialize_field("type", "table")?;
                t.serialize_field("data", c)?;
                t.end()
            }
        }
    }
}

#[derive(Serialize, Debug, Clone, PartialEq, Default)]
pub struct TableOfContentsReviewData {
    pub author: String,
    pub date: String,
}

/// https://c-rex.net/projects/samples/ooxml/e1/Part4/OOXML_P4_DOCX_TOCTOC_topic_ID0ELZO1.html
/// This struct is only used by writers
#[derive(Serialize, Debug, Clone, PartialEq, Default)]
pub struct TableOfContents {
    pub instr: InstrToC,
    pub items: Vec<TableOfContentsItem>,
    // don't use
    pub auto: bool,
    pub dirty: bool,
    /// Skip StructuredDataTag rendering
    pub without_sdt: bool,
    pub alias: Option<String>,
    pub page_ref_placeholder: Option<String>,
    // it is inserted in before toc.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub before_contents: Vec<TocContent>,
    // it is inserted in after toc.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub after_contents: Vec<TocContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<TableOfContentsReviewData>,
}

impl TableOfContents {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_instr_text(s: &str) -> Self {
        let instr = InstrToC::with_instr_text(s);
        Self {
            instr,
            ..Self::default()
        }
    }

    pub fn heading_styles_range(mut self, start: usize, end: usize) -> Self {
        self.instr = self.instr.heading_styles_range(start, end);
        self
    }

    pub fn add_style_with_level(mut self, s: StyleWithLevel) -> Self {
        self.instr = self.instr.add_style_with_level(s);
        self
    }

    pub fn hyperlink(mut self) -> Self {
        self.instr = self.instr.hyperlink();
        self
    }

    pub fn alias(mut self, a: impl Into<String>) -> Self {
        self.alias = Some(a.into());
        self
    }

    pub fn delete(mut self, author: impl Into<String>, date: impl Into<String>) -> Self {
        self.delete = Some(TableOfContentsReviewData {
            author: escape::escape(&author.into()),
            date: date.into(),
        });
        self
    }

    // pub fn tc_field_level_range(mut self, start: usize, end: usize) -> Self {
    //     self.instr = self.instr.tc_field_level_range(start, end);
    //     self
    // }

    pub fn add_item(mut self, t: TableOfContentsItem) -> Self {
        self.items.push(t);
        self
    }

    pub fn auto(mut self) -> Self {
        self.auto = true;
        self
    }

    pub fn dirty(mut self) -> Self {
        self.dirty = true;
        self
    }

    pub fn add_before_paragraph(mut self, p: Paragraph) -> Self {
        self.before_contents
            .push(TocContent::Paragraph(Box::new(p)));
        self
    }

    pub fn add_after_paragraph(mut self, p: Paragraph) -> Self {
        self.after_contents.push(TocContent::Paragraph(Box::new(p)));
        self
    }

    pub fn add_before_table(mut self, t: Table) -> Self {
        self.before_contents.push(TocContent::Table(Box::new(t)));
        self
    }

    pub fn add_after_table(mut self, t: Table) -> Self {
        self.after_contents.push(TocContent::Table(Box::new(t)));
        self
    }

    pub fn without_sdt(mut self) -> Self {
        self.without_sdt = true;
        self
    }

    fn inner_build(&self) -> Vec<u8> {
        let mut p = StructuredDataTagProperty::new();
        if let Some(ref alias) = self.alias {
            p = p.alias(alias);
        }
        if self.items.is_empty() {
            let mut b = XMLBuilder::new();

            if !self.without_sdt {
                b = b
                    .open_structured_tag()
                    .add_child(&p)
                    .open_structured_tag_content();
            }

            for c in self.before_contents.iter() {
                match c {
                    TocContent::Paragraph(p) => {
                        b = b.add_child(p);
                    }
                    TocContent::Table(t) => {
                        b = b.add_child(t);
                    }
                }
            }

            let p1 = if let Some(ref del) = self.delete {
                Paragraph::new().add_delete(
                    Delete::new().author(&del.author).date(&del.date).add_run(
                        Run::new()
                            .add_field_char(FieldCharType::Begin, true)
                            .add_delete_instr_text(DeleteInstrText::TOC(self.instr.clone()))
                            .add_field_char(FieldCharType::Separate, false),
                    ),
                )
            } else {
                Paragraph::new().add_run(
                    Run::new()
                        .add_field_char(FieldCharType::Begin, true)
                        .add_instr_text(InstrText::TOC(self.instr.clone()))
                        .add_field_char(FieldCharType::Separate, false),
                )
            };
            b = b.add_child(&p1);

            let p2 = Paragraph::new().add_run(Run::new().add_field_char(FieldCharType::End, false));
            if self.after_contents.is_empty() {
                b = b.add_child(&p2);
            } else {
                for (i, c) in self.after_contents.iter().enumerate() {
                    match c {
                        TocContent::Paragraph(p) => {
                            // Merge paragraph
                            if i == 0 {
                                let mut new_p = p.clone();
                                new_p.children.insert(
                                    0,
                                    ParagraphChild::Run(Box::new(
                                        Run::new().add_field_char(FieldCharType::End, false),
                                    )),
                                );
                                b = b.add_child(&new_p)
                            } else {
                                b = b.add_child(p);
                            }
                        }
                        TocContent::Table(t) => {
                            // insert empty line for table
                            // because it causes docx error if table is added after TOC
                            if i == 0 {
                                b = b.add_child(&Paragraph::new().add_run(Run::new().add_text("")));
                            }
                            b = b.add_child(t);
                        }
                    }
                }
            }

            if !self.without_sdt {
                b = b.close().close();
            }

            b.build()
        } else {
            let items: Vec<TableOfContentsItem> = self
                .items
                .iter()
                .map(|item| {
                    let mut item = item.clone();
                    item.instr = self.instr.clone();
                    item.dirty = self.dirty;
                    if item.page_ref.is_none() {
                        item.page_ref = self.page_ref_placeholder.clone();
                    }
                    item
                })
                .collect();

            let mut b = XMLBuilder::new();

            if !self.without_sdt {
                b = b
                    .open_structured_tag()
                    .add_child(&p)
                    .open_structured_tag_content();
            }

            for c in self.before_contents.iter() {
                match c {
                    TocContent::Paragraph(p) => {
                        b = b.add_child(p);
                    }
                    TocContent::Table(t) => {
                        b = b.add_child(t);
                    }
                }
            }

            b = b.add_child(&items);

            for (i, c) in self.after_contents.iter().enumerate() {
                match c {
                    TocContent::Paragraph(p) => {
                        b = b.add_child(p);
                    }
                    TocContent::Table(t) => {
                        // insert empty line for table
                        // because it causes docx error if table is added after TOC
                        if i == 0 {
                            b = b.add_child(&Paragraph::new().add_run(Run::new().add_text("")));
                        }
                        b = b.add_child(t);
                    }
                }
            }

            if !self.without_sdt {
                b = b.close().close();
            }
            b.build()
        }
    }
}

impl BuildXML for TableOfContents {
    fn build(&self) -> Vec<u8> {
        self.inner_build()
    }
}

impl BuildXML for Box<TableOfContents> {
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
    fn test_toc() {
        let b = TableOfContents::new().heading_styles_range(1, 3).build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:sdt><w:sdtPr><w:rPr /></w:sdtPr><w:sdtContent><w:p w14:paraId="12345678"><w:pPr><w:rPr /></w:pPr><w:r><w:rPr /><w:fldChar w:fldCharType="begin" w:dirty="true" /><w:instrText>TOC \o &quot;1-3&quot;</w:instrText><w:fldChar w:fldCharType="separate" w:dirty="false" /></w:r></w:p><w:p w14:paraId="12345678"><w:pPr><w:rPr /></w:pPr><w:r><w:rPr /><w:fldChar w:fldCharType="end" w:dirty="false" /></w:r></w:p></w:sdtContent>
</w:sdt>"#
        );
    }

    #[test]
    fn test_toc_without_sdt() {
        let b = TableOfContents::new()
            .without_sdt()
            .heading_styles_range(1, 3)
            .build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:p w14:paraId="12345678"><w:pPr><w:rPr /></w:pPr><w:r><w:rPr /><w:fldChar w:fldCharType="begin" w:dirty="true" /><w:instrText>TOC \o &quot;1-3&quot;</w:instrText><w:fldChar w:fldCharType="separate" w:dirty="false" /></w:r></w:p><w:p w14:paraId="12345678"><w:pPr><w:rPr /></w:pPr><w:r><w:rPr /><w:fldChar w:fldCharType="end" w:dirty="false" /></w:r></w:p>"#
        );
    }

    /*
        #[test]
        fn test_toc_with_items() {
            let b = TableOfContents::new()
                .heading_styles_range(1, 3)
                .add_items(Paragraph::new().add_run(Run::new().add_text("Hello")))
                .build();
            assert_eq!(
                str::from_utf8(&b).unwrap(),
                r#"<w:sdt>
      <w:sdtPr />
      <w:sdtContent><w:p w14:paraId="12345678"><w:pPr><w:rPr /></w:pPr><w:r><w:rPr /><w:fldChar w:fldCharType="begin" w:dirty="false" /><w:instrText>TOC \o &quot;1-3&quot;</w:instrText><w:fldChar w:fldCharType="separate" w:dirty="false" /></w:r><w:r><w:rPr /><w:t xml:space="preserve">Hello</w:t></w:r><w:r><w:rPr /><w:fldChar w:fldCharType="end" w:dirty="false" /></w:r></w:p></w:sdtContent>
    </w:sdt>"#
            );
        }
        */
}
