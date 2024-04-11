use serde::{Deserialize, Serialize, Serializer};

use crate::{xml_builder::XMLBuilder, BuildXML};

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Caps {
    val: bool,
}

impl Caps {
    pub fn new() -> Caps {
        Default::default()
    }

    pub fn disable(mut self) -> Caps {
        self.val = false;
        self
    }
}

impl Default for Caps {
    fn default() -> Self {
        Self { val: true }
    }
}

impl Serialize for Caps {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bool(self.val)
    }
}

impl BuildXML for Caps {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.caps(&self.val.to_string()).build()
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
        let c = Caps::new();
        let b = c.clone().build();
        assert_eq!(str::from_utf8(&b).unwrap(), r#"<w:caps w:val="true" />"#);

        let b = c.disable().build();
        assert_eq!(str::from_utf8(&b).unwrap(), r#"<w:caps w:val="false" />"#);
    }
}
