use crate::documents::BuildXML;
use crate::xml_builder::*;
use std::io::Write;

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HeaderReference {
    pub header_type: String,
    pub id: String,
}

impl Default for HeaderReference {
    fn default() -> HeaderReference {
        HeaderReference {
            header_type: "default".to_owned(),
            id: "rId4".to_owned(),
        }
    }
}

impl HeaderReference {
    pub fn new(t: impl Into<String>, id: impl Into<String>) -> HeaderReference {
        HeaderReference {
            header_type: t.into(),
            id: id.into(),
        }
    }
}

impl BuildXML for HeaderReference {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .header_reference(&self.header_type, &self.id)?
            .into_inner()
    }
}
