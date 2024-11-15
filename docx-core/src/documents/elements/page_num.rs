use serde::Serialize;
use std::io::Write;

use crate::documents::*;
use crate::types::*;

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
}

impl BuildXML for PageNum {
    fn build_to<W: Write>(
        &self,
        stream: xml::writer::EventWriter<W>,
    ) -> xml::writer::Result<xml::writer::EventWriter<W>> {
        Run::new()
            .add_field_char(FieldCharType::Begin, false)
            .add_instr_text(InstrText::PAGE(self.instr.clone()))
            .add_field_char(FieldCharType::Separate, false)
            .add_text("1")
            .add_field_char(FieldCharType::End, false)
            .build_to(stream)
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
            r#"<w:r><w:rPr /><w:fldChar w:fldCharType="begin" w:dirty="false" /><w:instrText>PAGE</w:instrText><w:fldChar w:fldCharType="separate" w:dirty="false" /><w:t xml:space="preserve">1</w:t><w:fldChar w:fldCharType="end" w:dirty="false" /></w:r>"#
        );
    }
}
