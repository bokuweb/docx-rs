use serde::{Deserialize, Serialize, Serializer};

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Vanish {}

impl Vanish {
    pub fn new() -> Vanish {
        Vanish {}
    }
}

impl Default for Vanish {
    fn default() -> Self {
        Vanish {}
    }
}

impl BuildXML for Vanish {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.vanish().build()
    }
}

impl Serialize for Vanish {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bool(true)
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
        let c = Vanish::new();
        let b = c.build();
        assert_eq!(str::from_utf8(&b).unwrap(), r#"<w:vanish />"#);
    }
}
