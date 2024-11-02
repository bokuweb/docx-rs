use serde::Serialize;
use std::io::Write;

use super::*;
use crate::documents::BuildXML;
// use crate::types::*;
use crate::xml_builder::*;

#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StructuredDataTagProperty {
    pub run_property: RunProperty,
    pub data_binding: Option<DataBinding>,
    pub alias: Option<String>,
}

impl Default for StructuredDataTagProperty {
    fn default() -> Self {
        Self {
            run_property: RunProperty::new(),
            data_binding: None,
            alias: None,
        }
    }
}

impl StructuredDataTagProperty {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn data_binding(mut self, d: DataBinding) -> Self {
        self.data_binding = Some(d);
        self
    }

    pub fn alias(mut self, v: impl Into<String>) -> Self {
        self.alias = Some(v.into());
        self
    }
}

impl BuildXML for StructuredDataTagProperty {
    fn build_to<W: Write>(
        &self,
        stream: xml::writer::EventWriter<W>,
    ) -> xml::writer::Result<xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_structured_tag_property()?
            .add_child(&self.run_property)?
            .add_optional_child(&self.data_binding)?
            .apply_opt(self.alias.as_ref(), |alias, b| b.alias(alias))?
            .close()?
            .into_inner()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_default() {
        let c = StructuredDataTagProperty::new();
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:sdtPr><w:rPr /></w:sdtPr>"#
        );
    }

    #[test]
    fn test_with_alias() {
        let c = StructuredDataTagProperty::new().alias("summary");
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:sdtPr><w:rPr /><w:alias w:val="summary" /></w:sdtPr>"#
        );
    }
}
