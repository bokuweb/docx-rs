use serde::Serialize;

use crate::documents::*;
use crate::types::*;
use crate::xml_builder::*;

// https://c-rex.net/projects/samples/ooxml/e1/Part4/OOXML_P4_DOCX_TOCTOC_topic_ID0ELZO1.html
#[derive(Serialize, Debug, Clone, PartialEq, Default)]
pub struct TableOfContents {
    //  If no heading range is specified, all heading levels used in the document are listed.
    heading_styles_range: Option<(usize, usize)>,
}

impl TableOfContents {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn heading_styles_range(mut self, start: usize, end: usize) -> Self {
        self.heading_styles_range = Some((start, end));
        self
    }

    fn build_instr_text(&self) -> String {
        let mut instr = "TOC".to_string();

        if let Some(heading_styles_range) = self.heading_styles_range {
            instr = format!(
                "{} \\o &quot;{}-{}&quot;",
                instr, heading_styles_range.0, heading_styles_range.1
            );
        }
        instr
    }
}

impl BuildXML for TableOfContents {
    fn build(&self) -> Vec<u8> {
        let p1 = Paragraph::new().add_run(
            Run::new()
                .add_field_char(FieldCharType::Begin, true)
                .add_instr_text(self.build_instr_text())
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
}
