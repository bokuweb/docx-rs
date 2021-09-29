use serde::Serialize;

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CoreProps {
    config: CorePropsConfig,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CorePropsConfig {
    created: Option<String>,
    creator: Option<String>,
    description: Option<String>,
    language: Option<String>,
    last_modified_by: Option<String>,
    modified: Option<String>,
    revision: Option<usize>,
    subject: Option<String>,
    title: Option<String>,
}

impl Default for CorePropsConfig {
    fn default() -> Self {
        Self {
            created: None,
            creator: None,
            description: None,
            language: None,
            last_modified_by: None,
            modified: None,
            revision: None,
            subject: None,
            title: None,
        }
    }
}

impl Default for CoreProps {
    fn default() -> Self {
        Self {
            config: CorePropsConfig::default(),
        }
    }
}

impl CoreProps {
    pub(crate) fn new(config: CorePropsConfig) -> CoreProps {
        CoreProps { config }
    }

    pub fn created_at(mut self, date: &str) -> Self {
        self.config.created = Some(date.to_owned());
        self
    }

    pub fn updated_at(mut self, date: &str) -> Self {
        self.config.modified = Some(date.to_owned());
        self
    }
}

impl CorePropsConfig {
    pub fn new() -> Self {
        CorePropsConfig {
            created: None,
            creator: None,
            description: None,
            language: None,
            last_modified_by: None,
            modified: None,
            revision: None,
            subject: None,
            title: None,
        }
    }
}

impl BuildXML for CoreProps {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        let base = b.declaration(Some(true)).open_core_properties(
            "http://schemas.openxmlformats.org/package/2006/metadata/core-properties",
            "http://purl.org/dc/elements/1.1/",
            "http://purl.org/dc/terms/",
            "http://purl.org/dc/dcmitype/",
            "http://www.w3.org/2001/XMLSchema-instance",
        );

        let convert = |v: usize| format!("{}", v);
        let mut base = base
            .dcterms_created(
                "dcterms:W3CDTF",
                self.config
                    .created
                    .as_ref()
                    .map_or_else(|| "1970-01-01T00:00:00Z", |v| v),
            )
            .dc_creator(
                self.config
                    .creator
                    .as_ref()
                    .map_or_else(|| "unknown", |v| v),
            )
            .cp_last_modified_by(
                self.config
                    .last_modified_by
                    .as_ref()
                    .map_or_else(|| "unknown", |v| v),
            )
            .dcterms_modified(
                "dcterms:W3CDTF",
                self.config
                    .modified
                    .as_ref()
                    .map_or_else(|| "1970-01-01T00:00:00Z", |v| v),
            )
            .cp_revision(&self.config.revision.map_or_else(|| "1".to_owned(), convert));
        if let Some(v) = self.config.description.as_ref() {
            base = base.dc_description(v);
        }
        if let Some(v) = self.config.language.as_ref() {
            base = base.dc_language(v);
        }
        if let Some(v) = self.config.subject.as_ref() {
            base = base.dc_subject(v);
        }
        if let Some(v) = self.config.title.as_ref() {
            base = base.dc_title(v);
        }
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
    fn test_default_doc_props_core() {
        let c = CoreProps::new(CorePropsConfig {
            created: None,
            creator: None,
            description: None,
            language: None,
            last_modified_by: None,
            modified: None,
            revision: None,
            subject: None,
            title: None,
        });
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<cp:coreProperties xmlns:cp="http://schemas.openxmlformats.org/package/2006/metadata/core-properties" xmlns:dc="http://purl.org/dc/elements/1.1/" xmlns:dcterms="http://purl.org/dc/terms/" xmlns:dcmitype="http://purl.org/dc/dcmitype/" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
  <dcterms:created xsi:type="dcterms:W3CDTF">1970-01-01T00:00:00Z</dcterms:created>
  <dc:creator>unknown</dc:creator>
  <cp:lastModifiedBy>unknown</cp:lastModifiedBy>
  <dcterms:modified xsi:type="dcterms:W3CDTF">1970-01-01T00:00:00Z</dcterms:modified>
  <cp:revision>1</cp:revision>
</cp:coreProperties>"#
        );
    }

    #[test]
    fn test_configured_doc_props_core_build() {
        let c = CoreProps::new(CorePropsConfig {
            created: Some("2019-01-01".to_owned()),
            creator: Some("foo".to_owned()),
            description: Some("bar".to_owned()),
            language: Some("en".to_owned()),
            last_modified_by: Some("go".to_owned()),
            modified: Some("2019-01-01".to_owned()),
            revision: Some(1),
            subject: Some("subject".to_owned()),
            title: Some("title".to_owned()),
        });
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<cp:coreProperties xmlns:cp="http://schemas.openxmlformats.org/package/2006/metadata/core-properties" xmlns:dc="http://purl.org/dc/elements/1.1/" xmlns:dcterms="http://purl.org/dc/terms/" xmlns:dcmitype="http://purl.org/dc/dcmitype/" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
  <dcterms:created xsi:type="dcterms:W3CDTF">2019-01-01</dcterms:created>
  <dc:creator>foo</dc:creator>
  <cp:lastModifiedBy>go</cp:lastModifiedBy>
  <dcterms:modified xsi:type="dcterms:W3CDTF">2019-01-01</dcterms:modified>
  <cp:revision>1</cp:revision>
  <dc:description>bar</dc:description>
  <dc:language>en</dc:language>
  <dc:subject>subject</dc:subject>
  <dc:title>title</dc:title>
</cp:coreProperties>"#
        );
    }
}
