use serde::Serialize;
use std::io::Write;

use crate::documents::*;
use crate::escape::escape;
use crate::xml_builder::XMLBuilder;

// https://c-rex.net/projects/samples/ooxml/e1/Part4/OOXML_P4_DOCX_TCTC_topic_ID0EU2N1.html
#[derive(Serialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct InstrTC {
    pub text: String,
    // \n Omits the page number for the entry.
    pub omits_page_number: bool,
    pub level: Option<usize>,
    // \f
    pub item_type_identifier: Option<String>,
}

impl InstrTC {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: escape(&text.into()),
            ..Default::default()
        }
    }

    pub fn omits_page_number(mut self) -> Self {
        self.omits_page_number = true;
        self
    }

    pub fn level(mut self, level: usize) -> Self {
        self.level = Some(level);
        self
    }

    pub fn item_type_identifier(mut self, t: impl Into<String>) -> Self {
        self.item_type_identifier = Some(t.into());
        self
    }
}

impl BuildXML for InstrTC {
    fn build_to<W: Write>(
        &self,
        stream: xml::writer::EventWriter<W>,
    ) -> xml::writer::Result<xml::writer::EventWriter<W>> {
        let mut b = XMLBuilder::from(stream);
        let raw = b.inner_mut()?;

        write!(raw, "TC \"{}\"", self.text)?;

        if let Some(ref t) = self.item_type_identifier {
            write!(raw, " \\f {}", t)?;
        }

        if let Some(level) = self.level {
            write!(raw, " \\l {}", level)?;
        }

        if self.omits_page_number {
            write!(raw, " \\n")?;
        }

        b.into_inner()
    }
}

fn parse_level(i: &str) -> Option<usize> {
    let r = i.replace("&quot;", "").replace('\"', "");
    if let Ok(l) = usize::from_str(&r) {
        return Some(l);
    }
    None
}

impl std::str::FromStr for InstrTC {
    type Err = ();

    fn from_str(instr: &str) -> Result<Self, Self::Err> {
        let mut s = instr.split(' ');
        let text = s.next();
        let mut tc = InstrTC::new(text.unwrap_or_default());
        loop {
            if let Some(i) = s.next() {
                match i {
                    "\\f" => {
                        if let Some(r) = s.next() {
                            let r = r.replace("&quot;", "").replace('\"', "");
                            tc = tc.item_type_identifier(r);
                        }
                    }
                    "\\l" => {
                        if let Some(r) = s.next() {
                            if let Some(l) = parse_level(r) {
                                tc = tc.level(l);
                            }
                        }
                    }
                    "\\n" => tc = tc.omits_page_number(),
                    _ => {}
                }
            } else {
                return Ok(tc);
            }
        }
    }
}
