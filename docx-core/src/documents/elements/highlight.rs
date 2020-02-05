use serde::{Deserialize, Serialize};

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Highlight {
    val: String,
}

impl Highlight {
    pub fn new(val: impl Into<String>) -> Highlight {
        Highlight { val: val.into() }
    }
}

impl BuildXML for Highlight {
    fn build(&self) -> Vec<u8> {
        XMLBuilder::new().highlight(&self.val).build()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_highlight() {
        let c = Highlight::new("FFFFFF");
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:highlight w:val="FFFFFF" />"#
        );
    }
}
