use serde::Serialize;

use crate::xml_builder::*;
use crate::{documents::BuildXML, RunProperty};

use super::run_property_default::*;
use super::RunFonts;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DocDefaults {
    run_property_default: RunPropertyDefault,
}

impl DocDefaults {
    pub fn new() -> DocDefaults {
        Default::default()
    }

    pub fn size(mut self, size: usize) -> Self {
        self.run_property_default = self.run_property_default.size(size);
        self
    }

    pub fn spacing(mut self, spacing: i32) -> Self {
        self.run_property_default = self.run_property_default.spacing(spacing);
        self
    }

    pub fn fonts(mut self, font: RunFonts) -> Self {
        self.run_property_default = self.run_property_default.fonts(font);
        self
    }

    pub(crate) fn run_property(mut self, p: RunProperty) -> Self {
        self.run_property_default = self.run_property_default.run_property(p);
        self
    }
}

impl Default for DocDefaults {
    fn default() -> Self {
        let run_property_default = RunPropertyDefault::new();
        DocDefaults {
            run_property_default,
        }
    }
}

impl BuildXML for DocDefaults {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.open_doc_defaults()
            .add_child(&self.run_property_default)
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
        let c = DocDefaults::new();
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:docDefaults><w:rPrDefault><w:rPr /></w:rPrDefault></w:docDefaults>"#
        );
    }
}
