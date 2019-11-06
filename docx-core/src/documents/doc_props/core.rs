use crate::documents::BuildXML;
use crate::xml_builder::*;

pub struct CoreProps {
    config: Option<CorePropsConfig>,
}

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

impl CoreProps {
    pub(crate) fn new(config: Option<CorePropsConfig>) -> CoreProps {
        CoreProps { config }
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
        let default = || "";

        if let Some(c) = &self.config {
            base.dcterms_created(
                "dcterms:W3CDTF",
                c.created.as_ref().map_or_else(default, |v| v),
            )
            .dc_creator(c.creator.as_ref().map_or_else(default, |v| v))
            .dc_description(c.description.as_ref().map_or_else(default, |v| v))
            .dc_language(c.language.as_ref().map_or_else(default, |v| v))
            .cp_last_modified_by(c.last_modified_by.as_ref().map_or_else(default, |v| v))
            .dcterms_modified(
                "dcterms:W3CDTF",
                c.modified.as_ref().map_or_else(default, |v| v),
            )
            .cp_revision(&c.revision.map_or_else(|| "".to_owned(), convert))
            .dc_subject(c.subject.as_ref().map_or_else(default, |v| v))
            .dc_title(c.title.as_ref().map_or_else(default, |v| v))
            .close()
            .build()
        } else {
            base.dcterms_created("dcterms:W3CDTF", "")
                .dc_creator("")
                .dc_description("")
                .dc_language("")
                .cp_last_modified_by("")
                .dcterms_modified("dcterms:W3CDTF", "")
                .cp_revision("")
                .dc_subject("")
                .dc_title("")
                .close()
                .build()
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::str;

    #[test]
    fn test_default_doc_props_core_build() {
        let c = CoreProps::new(None);
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<cp:coreProperties xmlns:cp="http://schemas.openxmlformats.org/package/2006/metadata/core-properties" xmlns:dc="http://purl.org/dc/elements/1.1/" xmlns:dcterms="http://purl.org/dc/terms/" xmlns:dcmitype="http://purl.org/dc/dcmitype/" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
  <dcterms:created xsi:type="dcterms:W3CDTF"></dcterms:created>
  <dc:creator></dc:creator>
  <dc:description></dc:description>
  <dc:language></dc:language>
  <cp:lastModifiedBy></cp:lastModifiedBy>
  <dcterms:modified xsi:type="dcterms:W3CDTF"></dcterms:modified>
  <cp:revision></cp:revision>
  <dc:subject></dc:subject>
  <dc:title></dc:title>
</cp:coreProperties>"#
        );
    }

    #[test]
    fn test_configured_doc_props_core_build() {
        let c = CoreProps::new(Some(CorePropsConfig {
            created: Some("2019-01-01".to_owned()),
            creator: Some("foo".to_owned()),
            description: Some("bar".to_owned()),
            language: Some("en".to_owned()),
            last_modified_by: Some("go".to_owned()),
            modified: Some("2019-01-01".to_owned()),
            revision: Some(1),
            subject: Some("subject".to_owned()),
            title: Some("title".to_owned()),
        }));
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<cp:coreProperties xmlns:cp="http://schemas.openxmlformats.org/package/2006/metadata/core-properties" xmlns:dc="http://purl.org/dc/elements/1.1/" xmlns:dcterms="http://purl.org/dc/terms/" xmlns:dcmitype="http://purl.org/dc/dcmitype/" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
  <dcterms:created xsi:type="dcterms:W3CDTF">2019-01-01</dcterms:created>
  <dc:creator>foo</dc:creator>
  <dc:description>bar</dc:description>
  <dc:language>en</dc:language>
  <cp:lastModifiedBy>go</cp:lastModifiedBy>
  <dcterms:modified xsi:type="dcterms:W3CDTF">2019-01-01</dcterms:modified>
  <cp:revision>1</cp:revision>
  <dc:subject>subject</dc:subject>
  <dc:title>title</dc:title>
</cp:coreProperties>"#
        );
    }
}
