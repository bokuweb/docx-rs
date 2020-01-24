use super::RunProperty;
use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug)]
pub struct RunPropertyDefault {
    run_property: RunProperty,
}

impl RunPropertyDefault {
    pub fn new() -> RunPropertyDefault {
        Default::default()
    }
}

impl Default for RunPropertyDefault {
    fn default() -> Self {
        let run_property = RunProperty::new();
        RunPropertyDefault { run_property }
    }
}

impl BuildXML for RunPropertyDefault {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.open_run_property_default()
            .add_child(&self.run_property)
            .close()
            .build()
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
        let c = RunPropertyDefault::new();
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:rPrDefault><w:rPr /></w:rPrDefault>"#
        );
    }
}
