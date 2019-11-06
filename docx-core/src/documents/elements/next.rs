use crate::documents::BuildXML;
use crate::xml_builder::*;

pub struct Next {
    val: String,
}

impl Next {
    pub fn new(val: impl Into<String>) -> Next {
        Next { val: val.into() }
    }
}

impl BuildXML for Next {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.next(&self.val).build()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_next() {
        let c = Next::new("Normal");
        let b = c.build();
        assert_eq!(str::from_utf8(&b).unwrap(), r#"<w:next w:val="Normal" />"#);
    }
}
