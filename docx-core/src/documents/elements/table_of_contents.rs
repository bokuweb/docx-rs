use serde::Serialize;

use crate::documents::*;
use crate::types::*;
use crate::xml_builder::*;

// https://c-rex.net/projects/samples/ooxml/e1/Part4/OOXML_P4_DOCX_TOCTOC_topic_ID0ELZO1.html
// This struct is only used by writers
#[derive(Serialize, Debug, Clone, PartialEq, Default)]
pub struct TableOfContents {
    pub instr: InstrToC,
    pub items: Vec<Paragraph>,
}

impl TableOfContents {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn heading_styles_range(mut self, start: usize, end: usize) -> Self {
        self.instr = self.instr.heading_styles_range(start, end);
        self
    }

    pub fn add_items(mut self, p: Paragraph) -> Self {
        self.items.push(p);
        self
    }
}

impl BuildXML for TableOfContents {
    fn build(&self) -> Vec<u8> {
        if self.items.is_empty() {
            let p1 = Paragraph::new().add_run(
                Run::new()
                    .add_field_char(FieldCharType::Begin, true)
                    .add_instr_text(InstrText::TOC(self.instr.clone()))
                    .add_field_char(FieldCharType::Separate, false),
            );
            let p2 = Paragraph::new().add_run(Run::new().add_field_char(FieldCharType::End, false));

            XMLBuilder::new()
                .open_structured_tag()
                .open_structured_tag_property()
                .close()
                .open_structured_tag_content()
                .add_child(&p1)
                .add_child(&p2)
                .close()
                .close()
                .build()
        } else {
            let mut b = XMLBuilder::new()
                .open_structured_tag()
                .open_structured_tag_property()
                .close()
                .open_structured_tag_content();

            for (i, p) in self.items.iter().enumerate() {
                if i == 0 {
                    let mut p = p.clone().unshift_run(
                        Run::new()
                            .add_field_char(FieldCharType::Begin, false)
                            .add_instr_text(InstrText::TOC(self.instr.clone()))
                            .add_field_char(FieldCharType::Separate, false),
                    );
                    if i == self.items.len() - 1 {
                        p = p.add_run(Run::new().add_field_char(FieldCharType::End, false));
                    }
                    b = b.add_child(&p);
                } else if i == self.items.len() - 1 {
                    let p = p
                        .clone()
                        .add_run(Run::new().add_field_char(FieldCharType::End, false));
                    b = b.add_child(&p);
                } else {
                    b = b.add_child(p);
                }
            }
            b.close().close().build()
        }
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
            r#"<w:sdt>
  <w:sdtPr />
  <w:sdtContent><w:p w14:paraId="12345678"><w:pPr><w:rPr /></w:pPr><w:r><w:rPr /><w:fldChar w:fldCharType="begin" w:dirty="true" /><w:instrText>TOC \o &quot;1-3&quot;</w:instrText><w:fldChar w:fldCharType="separate" w:dirty="false" /></w:r></w:p><w:p w14:paraId="12345678"><w:pPr><w:rPr /></w:pPr><w:r><w:rPr /><w:fldChar w:fldCharType="end" w:dirty="false" /></w:r></w:p></w:sdtContent>
</w:sdt>"#
        );
    }

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
}
