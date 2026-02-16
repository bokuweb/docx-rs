use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;
use std::io::Write;

use crate::documents::*;
use crate::xml_builder::*;

#[derive(Debug, Clone, PartialEq)]
pub enum DeleteInstrText {
    TOC(InstrToC),
    TC(InstrTC),
    PAGEREF(InstrPAGEREF),
    HYPERLINK(InstrHyperlink),
    Unsupported(String),
}

impl BuildXML for DeleteInstrText {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_delete_instr_text()?
            .apply(|b| match self {
                DeleteInstrText::TOC(toc) => b.add_child(toc),
                DeleteInstrText::TC(tc) => b.add_child(tc),
                DeleteInstrText::PAGEREF(page_ref) => b.add_child(page_ref),
                DeleteInstrText::HYPERLINK(_link) => todo!(),
                DeleteInstrText::Unsupported(s) => b.plain_text(s),
            })?
            .close()?
            .into_inner()
    }
}

impl Serialize for DeleteInstrText {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            DeleteInstrText::TOC(ref s) => {
                let mut t = serializer.serialize_struct("TOC", 2)?;
                t.serialize_field("type", "toc")?;
                t.serialize_field("data", s)?;
                t.end()
            }
            DeleteInstrText::TC(ref s) => {
                let mut t = serializer.serialize_struct("TC", 2)?;
                t.serialize_field("type", "tc")?;
                t.serialize_field("data", s)?;
                t.end()
            }
            DeleteInstrText::PAGEREF(ref s) => {
                let mut t = serializer.serialize_struct("PAGEREF", 2)?;
                t.serialize_field("type", "pageref")?;
                t.serialize_field("data", s)?;
                t.end()
            }
            DeleteInstrText::HYPERLINK(ref s) => {
                let mut t = serializer.serialize_struct("HYPERLINK", 2)?;
                t.serialize_field("type", "hyperlink")?;
                t.serialize_field("data", s)?;
                t.end()
            }
            DeleteInstrText::Unsupported(ref s) => {
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
    fn test_delete_toc_instr() {
        let b = DeleteInstrText::TOC(InstrToC::new().heading_styles_range(1, 3)).build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:delInstrText>TOC \o &quot;1-3&quot;</w:delInstrText>"#
        );
    }
}
