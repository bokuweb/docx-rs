use serde::{Serialize, Serializer};

use crate::{xml_builder::XMLBuilder, BuildXML};

#[derive(Debug, Clone, PartialEq)]
pub struct UiPriority {
    val: i32,
}

impl UiPriority {
    pub fn new(val: i32) -> UiPriority {
        UiPriority { val }
    }
}

impl Serialize for UiPriority {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.val)
    }
}

impl BuildXML for UiPriority {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.ui_priority(self.val).build()
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
        let c = UiPriority::new(1);
        let b = c.build();
        assert_eq!(str::from_utf8(&b).unwrap(), r#"<w:uiPriority w:val="1" />"#);
    }
}
