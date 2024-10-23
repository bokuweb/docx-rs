use serde::{Serialize, Serializer};
use std::io::Write;

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, PartialEq)]
pub struct AdjustRightInd(pub isize);

impl AdjustRightInd {
    pub fn new(val: isize) -> AdjustRightInd {
        AdjustRightInd(val)
    }
}

impl BuildXML for AdjustRightInd {
    fn build_to<W: Write>(
        &self,
        stream: xml::writer::EventWriter<W>,
    ) -> xml::writer::Result<xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .adjust_right_ind(self.0)?
            .into_inner()
    }
}

impl Serialize for AdjustRightInd {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(self.0 as i64)
    }
}
