use crate::documents::BuildXML;
use crate::xml_builder::*;

use serde::{Serialize, Serializer};

#[derive(Debug, Clone, PartialEq)]
pub struct Zoom {
    val: usize,
}

impl Zoom {
    pub fn new(val: usize) -> Zoom {
        Zoom { val }
    }
}

impl BuildXML for Zoom {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.zoom(&format!("{}", self.val)).build()
    }
}

impl Serialize for Zoom {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.val as u64)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_zoom() {
        let c = Zoom::new(20);
        let b = c.build();
        assert_eq!(str::from_utf8(&b).unwrap(), r#"<w:zoom w:percent="20" />"#);
    }
}
