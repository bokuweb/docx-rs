use serde::Serialize;

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppProps {}

impl AppProps {
    pub fn new() -> AppProps {
        Default::default()
    }
}

impl Default for AppProps {
    fn default() -> Self {
        Self {}
    }
}

impl BuildXML for AppProps {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        let base = b.declaration(Some(true)).open_properties(
            "http://schemas.openxmlformats.org/officeDocument/2006/extended-properties",
            "http://schemas.openxmlformats.org/officeDocument/2006/docPropsVTypes",
        );
        base.close().build()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_default_doc_props_app_build() {
        let c = AppProps::new();
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Properties xmlns="http://schemas.openxmlformats.org/officeDocument/2006/extended-properties" xmlns:vt="http://schemas.openxmlformats.org/officeDocument/2006/docPropsVTypes" />"#
        );
    }
}
