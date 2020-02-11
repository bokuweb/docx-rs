use serde::{Serialize, Serializer};

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, PartialEq)]
pub struct GridSpan {
    val: usize,
}

impl GridSpan {
    pub fn new(v: usize) -> GridSpan {
        GridSpan { val: v }
    }
}

impl BuildXML for GridSpan {
    fn build(&self) -> Vec<u8> {
        XMLBuilder::new().grid_span(self.val).build()
    }
}

impl Serialize for GridSpan {
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
    fn test_grid_span() {
        let b = GridSpan::new(3).build();
        assert_eq!(str::from_utf8(&b).unwrap(), r#"<w:gridSpan w:val="3" />"#);
    }
}
