use serde::{Serialize, Serializer};

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, PartialEq)]
pub struct LevelRestart {
    val: u32,
}

impl LevelRestart {
    pub fn new(val: impl Into<u32>) -> Self {
        Self { val: val.into() }
    }
}

impl BuildXML for LevelRestart {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        let v = format!("{}", &self.val);
        b.level_restart(&v).build()
    }
}

impl Serialize for LevelRestart {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u32(self.val)
    }
}
