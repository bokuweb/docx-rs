use serde::Serialize;

use crate::documents::*;
use crate::xml_builder::*;

use super::instrs::toc::*;

#[derive(Serialize, Debug, Clone, PartialEq)]
pub enum InstrText {
    ToC(ToC),
    Unsupported,
}

impl BuildXML for InstrText {
    fn build(&self) -> Vec<u8> {
        if self == &InstrText::Unsupported {
            return vec![];
        }
        XMLBuilder::new()
            .open_instr_text()
            .add_child(match self {
                Self::ToC(toc) => toc,
                _ => unreachable!(),
            })
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
        let b = InstrText::ToC(ToC::new().heading_styles_range(1, 3)).build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:instrText>ToC \o "1-3"</w:instrText>"#
        );
    }
}
