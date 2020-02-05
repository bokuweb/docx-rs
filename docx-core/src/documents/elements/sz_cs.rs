use crate::documents::BuildXML;
use crate::xml_builder::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SzCs {
    val: usize,
}

impl SzCs {
    pub fn new(val: usize) -> SzCs {
        SzCs { val }
    }
}

impl BuildXML for SzCs {
    fn build(&self) -> Vec<u8> {
        XMLBuilder::new().sz_cs(self.val).build()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_sz_cs() {
        let c = SzCs::new(20);
        let b = c.build();
        assert_eq!(str::from_utf8(&b).unwrap(), r#"<w:szCs w:val="20" />"#);
    }
}
