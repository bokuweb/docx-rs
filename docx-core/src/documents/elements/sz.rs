use serde::{Deserialize, Serialize, Serializer};

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Sz {
    val: usize,
}

impl Sz {
    pub fn new(val: usize) -> Sz {
        Sz { val }
    }
}

impl BuildXML for Sz {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new(Vec::new());
        b.sz(self.val).into_inner()
    }
}

impl Serialize for Sz {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u32(self.val as u32)
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
