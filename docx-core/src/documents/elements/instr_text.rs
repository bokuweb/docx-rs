use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;

use crate::documents::*;
use crate::xml_builder::*;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "wasm", derive(ts_rs::TS))]
#[cfg_attr(feature = "wasm", ts(export))]
pub enum InstrText {
    TOC(InstrToC),
    TC(InstrTC),
    PAGEREF(InstrPAGEREF),
    HYPERLINK(InstrHyperlink),
    Unsupported(String),
}

impl BuildXML for Box<InstrText> {
    fn build(&self) -> Vec<u8> {
        let instr = match self.as_ref() {
            InstrText::TOC(toc) => toc.build(),
            InstrText::TC(tc) => tc.build(),
            InstrText::PAGEREF(page_ref) => page_ref.build(),
            InstrText::HYPERLINK(_link) => todo!(),
            InstrText::Unsupported(s) => s.as_bytes().to_vec(),
        };
        XMLBuilder::new()
            .open_instr_text()
            .add_bytes(&instr)
            .close()
            .build()
    }
}

impl Serialize for InstrText {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            InstrText::TOC(ref s) => {
                let mut t = serializer.serialize_struct("TOC", 2)?;
                t.serialize_field("type", "toc")?;
                t.serialize_field("data", s)?;
                t.end()
            }
            InstrText::TC(ref s) => {
                let mut t = serializer.serialize_struct("TC", 2)?;
                t.serialize_field("type", "tc")?;
                t.serialize_field("data", s)?;
                t.end()
            }
            InstrText::PAGEREF(ref s) => {
                let mut t = serializer.serialize_struct("PAGEREF", 2)?;
                t.serialize_field("type", "pageref")?;
                t.serialize_field("data", s)?;
                t.end()
            }
            InstrText::HYPERLINK(ref s) => {
                let mut t = serializer.serialize_struct("HYPERLINK", 2)?;
                t.serialize_field("type", "hyperlink")?;
                t.serialize_field("data", s)?;
                t.end()
            }
            InstrText::Unsupported(ref s) => {
                let mut t = serializer.serialize_struct("Unsupported", 2)?;
                t.serialize_field("type", "unsupported")?;
                t.serialize_field("data", s)?;
                t.end()
            }
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
    fn test_toc_instr() {
        let b = Box::new(InstrText::TOC(InstrToC::new().heading_styles_range(1, 3))).build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:instrText>TOC \o &quot;1-3&quot;</w:instrText>"#
        );
    }

    #[test]
    fn test_pageref_instr() {
        let b = Box::new(InstrText::PAGEREF(
            InstrPAGEREF::new("_Toc90425847").hyperlink(),
        ))
        .build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:instrText>PAGEREF _Toc90425847 \h</w:instrText>"#
        );
    }
}
