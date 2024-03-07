use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;

use crate::documents::*;
use crate::types::*;
use crate::xml_builder::*;

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct PageNum {
    pub instr: InstrPAGE,
}

impl Default for PageNum {
    fn default() -> Self {
        Self {
            instr: InstrPAGE {},
        }
    }
}

impl PageNum {
    pub fn new() -> Self {
        Self::default()
    }

    fn inner_build(&self) -> Vec<u8> {
        let p = StructuredDataTagProperty::new();
        let mut b = XMLBuilder::new();

        b = b
            .open_structured_tag()
            .add_child(&p)
            .open_structured_tag_content();

        let p = Paragraph::new().add_run(
            Run::new()
                .add_field_char(FieldCharType::Begin, false)
                .add_instr_text(InstrText::PAGE(self.instr.clone()))
                .add_field_char(FieldCharType::Separate, false)
                .add_text("1")
                .add_field_char(FieldCharType::End, false),
        );
        b = b.add_child(&p);
        b = b.close().close();
        b.build()
    }
}

impl BuildXML for PageNum {
    fn build(&self) -> Vec<u8> {
        self.inner_build()
    }
}

impl BuildXML for Box<PageNum> {
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
    fn test_page() {
        let b = PageNum::new().build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:sdt><w:sdtPr><w:rPr /></w:sdtPr><w:sdtContent><w:p w14:paraId="12345678"><w:pPr><w:rPr /></w:pPr><w:r><w:rPr /><w:fldChar w:fldCharType="begin" w:dirty="false" /><w:instrText>PAGE</w:instrText><w:fldChar w:fldCharType="separate" w:dirty="false" /><w:t xml:space="preserve">1</w:t><w:fldChar w:fldCharType="end" w:dirty="false" /></w:r></w:p></w:sdtContent>
</w:sdt>"#
        );
    }
}
