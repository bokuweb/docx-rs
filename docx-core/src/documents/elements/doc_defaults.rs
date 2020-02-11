use serde::Serialize;

use crate::documents::BuildXML;
use crate::xml_builder::*;

use super::run_property_default::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DocDefaults {
    run_property_default: RunPropertyDefault,
}

impl DocDefaults {
    pub fn new() -> DocDefaults {
        Default::default()
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
