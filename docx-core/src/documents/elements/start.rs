use serde::{Serialize, Serializer};

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Start {
    val: usize,
}

impl Start {
    pub fn new(val: usize) -> Start {
        Start { val }
    }
}

impl Default for Start {
    fn default() -> Self {
        Start { val: 0 }
    }
}

impl BuildXML for Start {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.start(self.val).build()
    }
}

impl Serialize for Start {
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
    fn test_start() {
        let c = Start::new(1);
        let b = c.build();
        assert_eq!(str::from_utf8(&b).unwrap(), r#"<w:start w:val="1" />"#);
    }
}
