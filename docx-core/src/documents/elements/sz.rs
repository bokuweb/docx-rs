use crate::documents::BuildXML;
use crate::xml_builder::*;

use super::Name;

pub struct Sz {
    val: usize,
}

impl Sz {
    pub fn new(val: usize) -> Sz {
        Sz { val }
    }
}

impl BuildXML for Sz {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.sz(self.val).build()
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
        let c = Sz::new(20);
        let b = c.build();
        assert_eq!(str::from_utf8(&b).unwrap(), r#"<w:sz w:val="20" />"#);
    }
}
