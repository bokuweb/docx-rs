use crate::documents::BuildXML;
use crate::xml_builder::*;
use serde::{Deserialize, Serialize, Serializer};

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "wasm", derive(ts_rs::TS))]
#[cfg_attr(feature = "wasm", ts(export))]
pub struct SzCs(pub usize);

impl SzCs {
    pub fn new(val: usize) -> Self {
        Self(val)
    }
}

impl BuildXML for SzCs {
    fn build(&self) -> Vec<u8> {
        XMLBuilder::new().sz_cs(self.0).build()
    }
}

impl Serialize for SzCs {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u32(self.0 as u32)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_sz_cs() {
        let c = SzCs::new(20);
        let b = c.build();
        assert_eq!(str::from_utf8(&b).unwrap(), r#"<w:szCs w:val="20" />"#);
    }
}
