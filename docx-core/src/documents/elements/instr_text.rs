use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;
use std::io::Write;

use crate::documents::*;
use crate::xml_builder::*;

#[derive(Debug, Clone, PartialEq)]
pub enum InstrText {
    TOC(InstrToC),
    TC(InstrTC),
    PAGE(InstrPAGE),
    NUMPAGES(InstrNUMPAGES),
    PAGEREF(InstrPAGEREF),
    HYPERLINK(InstrHyperlink),
    Unsupported(String),
}

impl BuildXML for Box<InstrText> {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_instr_text()?
            .apply(|b| match self.as_ref() {
                InstrText::TOC(toc) => b.add_child(toc),
                InstrText::TC(tc) => b.add_child(tc),
                InstrText::PAGEREF(page_ref) => b.add_child(page_ref),
                InstrText::PAGE(page) => b.add_child(page),
                InstrText::NUMPAGES(page) => b.add_child(page),
                InstrText::HYPERLINK(_link) => todo!(),
                InstrText::Unsupported(s) => b.plain_text(s),
            })?
            .close()?
            .into_inner()
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
            InstrText::PAGE(ref s) => {
                let mut t = serializer.serialize_struct("PAGE", 2)?;
                t.serialize_field("type", "page")?;
                t.serialize_field("data", s)?;
                t.end()
            }
            InstrText::NUMPAGES(ref s) => {
                let mut t = serializer.serialize_struct("NUMPAGES", 2)?;
                t.serialize_field("type", "numPages")?;
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

#[allow(unused_allocation)]
#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_toc_instr() {
        #[allow(unused_allocation)]
        let b = Box::new(InstrText::TOC(InstrToC::new().heading_styles_range(1, 3))).build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:instrText>TOC \o &quot;1-3&quot;</w:instrText>"#
        );
    }

    #[test]
    fn test_pageref_instr() {
        #[allow(unused_allocation)]
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
