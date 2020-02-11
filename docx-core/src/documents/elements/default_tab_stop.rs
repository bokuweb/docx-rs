use crate::documents::BuildXML;
use crate::xml_builder::*;

use serde::{Serialize, Serializer};

#[derive(Debug, Clone, PartialEq)]
pub struct DefaultTabStop {
    val: usize,
}

impl DefaultTabStop {
    pub fn new(val: usize) -> DefaultTabStop {
        DefaultTabStop { val }
    }
}

impl BuildXML for DefaultTabStop {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.default_tab_stop(self.val).build()
    }
}

impl Serialize for DefaultTabStop {
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
        let c = DefaultTabStop::new(20);
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:defaultTabStop w:val="20" />"#
        );
    }
}
