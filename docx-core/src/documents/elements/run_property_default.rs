use serde::Serialize;

use super::*;
use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RunPropertyDefault {
    run_property: RunProperty,
}

impl RunPropertyDefault {
    pub fn new() -> RunPropertyDefault {
        Default::default()
    }

    pub fn size(mut self, size: usize) -> Self {
        self.run_property = self.run_property.size(size);
        self
    }

    pub fn spacing(mut self, spacing: i32) -> Self {
        self.run_property = self.run_property.spacing(spacing);
        self
    }

    pub fn fonts(mut self, font: RunFonts) -> Self {
        self.run_property = self.run_property.fonts(font);
        self
    }

    pub(crate) fn run_property(mut self, p: RunProperty) -> Self {
        self.run_property = p;
        self
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
