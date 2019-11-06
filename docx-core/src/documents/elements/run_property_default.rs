use crate::documents::BuildXML;
use crate::xml_builder::*;

pub struct RunPropertyDefault {}

impl RunPropertyDefault {
    pub fn new() -> RunPropertyDefault {
        RunPropertyDefault {}
    }
}

impl BuildXML for RunPropertyDefault {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.open_run_property_default().close().build()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::str;

    #[test]
    fn test_build() {
        let c = RunPropertyDefault::new();
        let b = c.build();
        assert_eq!(str::from_utf8(&b).unwrap(), r#"<w:rPrDefault />"#);
    }
}
