use serde::{Serialize, Serializer};

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, PartialEq)]
pub struct BasedOn {
    val: String,
}

impl BasedOn {
    pub fn new(val: impl Into<String>) -> BasedOn {
        BasedOn { val: val.into() }
    }
}

impl Serialize for BasedOn {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.val)
    }
}

impl BuildXML for BasedOn {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.based_on(&self.val).build()
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
        let c = BasedOn::new("Normal");
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:basedOn w:val="Normal" />"#
        );
    }
}
