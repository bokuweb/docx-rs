use serde::{Serialize, Serializer};

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
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.adjust_right_ind(self.0).build()
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
