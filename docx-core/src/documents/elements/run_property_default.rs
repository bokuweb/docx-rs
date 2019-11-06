use super::RunProperty;
use crate::documents::BuildXML;
use crate::xml_builder::*;

pub struct RunPropertyDefault {
    run_property: RunProperty,
}

impl RunPropertyDefault {
    pub fn new() -> RunPropertyDefault {
        let run_property = RunProperty::new();
        RunPropertyDefault { run_property }
    }
}

impl BuildXML for RunPropertyDefault {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        let run_property = self.run_property.build();
        b.open_run_property_default()
            .add_child_buffer(&run_property)
            .close()
            .build()
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
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:rPrDefault><w:rPr /></w:rPrDefault>"#
        );
    }
}
