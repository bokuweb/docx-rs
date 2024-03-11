use serde::{Deserialize, Serialize, Serializer};

use crate::documents::BuildXML;
use crate::xml_builder::*;
use ts_rs::*;

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[cfg_attr(feature = "wasm", derive(TS))]
#[cfg_attr(feature = "wasm", ts(export))]
pub struct Sz(usize);

impl Sz {
    pub fn new(val: usize) -> Sz {
        Sz(val)
    }
}

impl BuildXML for Sz {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.sz(self.0).build()
    }
}

impl Serialize for Sz {
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
    fn test_build() {
        let c = Sz::new(20);
        let b = c.build();
        assert_eq!(str::from_utf8(&b).unwrap(), r#"<w:sz w:val="20" />"#);
    }
}
