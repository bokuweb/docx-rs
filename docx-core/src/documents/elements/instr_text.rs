use serde::Serialize;

use crate::documents::*;
use crate::xml_builder::*;

#[derive(Serialize, Debug, Clone, PartialEq)]
pub enum InstrText {
    TOC(TableOfContents),
    Unsupported(String),
}

impl BuildXML for InstrText {
    fn build(&self) -> Vec<u8> {
        let instr = match self {
            InstrText::TOC(toc) => toc.build_instr_text(),
            InstrText::Unsupported(s) => s.to_string(),
        };
        XMLBuilder::new()
            .open_instr_text()
            .plain_text(&instr)
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
    fn test_toc_instr() {
        let b = InstrText::TOC(TableOfContents::new().heading_styles_range(1, 3)).build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:instrText>TOC \o &quot;1-3&quot;</w:instrText>"#
        );
    }
}
