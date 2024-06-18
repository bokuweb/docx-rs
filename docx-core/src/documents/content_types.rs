use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::io::Read;
use xml::reader::{EventReader, XmlEvent};

use crate::documents::BuildXML;
use crate::reader::{FromXML, ReaderError};
use crate::xml_builder::*;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ContentTypes {
    types: BTreeMap<String, String>,
    web_extension_count: usize,
    custom_xml_count: usize,
    header_count: usize,
    footer_count: usize,
}

impl ContentTypes {
    pub fn new() -> ContentTypes {
        Default::default()
    }

    pub fn add_content(mut self, path: impl Into<String>, namespace: impl Into<String>) -> Self {
        self.types.insert(path.into(), namespace.into());
        self
    }

    pub fn set_default(mut self) -> ContentTypes {
        self.types.insert(
            "/_rels/.rels".to_owned(),
            "application/vnd.openxmlformats-package.relationships+xml".to_owned(),
        );
        self.types.insert(
            "/docProps/app.xml".to_owned(),
            "application/vnd.openxmlformats-officedocument.extended-properties+xml".to_owned(),
        );
        self.types.insert(
            "/docProps/core.xml".to_owned(),
            "application/vnd.openxmlformats-package.core-properties+xml".to_owned(),
        );
        self.types.insert(
            "/word/_rels/document.xml.rels".to_owned(),
            "application/vnd.openxmlformats-package.relationships+xml".to_owned(),
        );
        self.types.insert(
            "/word/settings.xml".to_owned(),
            "application/vnd.openxmlformats-officedocument.wordprocessingml.settings+xml"
                .to_owned(),
        );
        self.types.insert(
            "/word/fontTable.xml".to_owned(),
            "application/vnd.openxmlformats-officedocument.wordprocessingml.fontTable+xml"
                .to_owned(),
        );
        self.types.insert(
            "/word/document.xml".to_owned(),
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"
                .to_owned(),
        );
        self.types.insert(
            "/word/styles.xml".to_owned(),
            "application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml".to_owned(),
        );
        self.types.insert(
            "/word/comments.xml".to_owned(),
            "application/vnd.openxmlformats-officedocument.wordprocessingml.comments+xml"
                .to_owned(),
        );
        self.types.insert(
            "/word/numbering.xml".to_owned(),
            "application/vnd.openxmlformats-officedocument.wordprocessingml.numbering+xml"
                .to_owned(),
        );
        self.types.insert(
            "/word/commentsExtended.xml".to_owned(),
            "application/vnd.openxmlformats-officedocument.wordprocessingml.commentsExtended+xml"
                .to_owned(),
        );
        self.types.insert(
            "/docProps/custom.xml".to_owned(),
            "application/vnd.openxmlformats-officedocument.custom-properties+xml".to_owned(),
        );
        self
    }

    pub fn add_taskpanes(mut self) -> Self {
        self.types.insert(
            "/word/webextensions/taskpanes.xml".to_owned(),
            "application/vnd.ms-office.webextensiontaskpanes+xml".to_owned(),
        );
        self
    }

    pub fn add_web_extensions(mut self) -> Self {
        self.types.insert(
            format!(
                "/word/webextensions/webextension{}.xml",
                self.web_extension_count
            ),
            "application/vnd.ms-office.webextension+xml".to_owned(),
        );
        self.web_extension_count += 1;
        self
    }

    pub fn add_custom_xml(mut self) -> Self {
        self.types.insert(
            format!("/customXml/itemProps{}.xml", self.web_extension_count),
            "application/vnd.openxmlformats-officedocument.customXmlProperties+xml".to_owned(),
        );
        self.custom_xml_count += 1;
        self
    }

    pub fn add_header(mut self) -> Self {
        self.header_count += 1;
        self.types.insert(
            format!("/word/header{}.xml", self.header_count),
            "application/vnd.openxmlformats-officedocument.wordprocessingml.header+xml".to_owned(),
        );
        self
    }

    pub fn add_footer(mut self) -> Self {
        self.footer_count += 1;
        self.types.insert(
            format!("/word/footer{}.xml", self.footer_count),
            "application/vnd.openxmlformats-officedocument.wordprocessingml.footer+xml".to_owned(),
        );
        self
    }
    pub fn add_footnotes(mut self) -> Self {
        self.types.insert(
            "/word/footnotes.xml".to_owned(),
            "application/vnd.openxmlformats-officedocument.wordprocessingml.footnotes+xml"
                .to_owned(),
        );
        self
    }
}

impl Default for ContentTypes {
    fn default() -> Self {
        ContentTypes {
            types: BTreeMap::new(),
            web_extension_count: 1,
            custom_xml_count: 1,
            header_count: 0,
            footer_count: 0,
        }
    }
}

impl BuildXML for ContentTypes {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        let mut b = b
            .declaration(None)
            .open_types("http://schemas.openxmlformats.org/package/2006/content-types");

        b = b
            .add_default("png", "image/png")
            .add_default("jpeg", "image/jpeg")
            .add_default("jpg", "image/jpg")
            .add_default("bmp", "image/bmp")
            .add_default("gif", "image/gif")
            .add_default(
                "rels",
                "application/vnd.openxmlformats-package.relationships+xml",
            )
            .add_default("xml", "application/xml");

        for (k, v) in self.types.iter() {
            b = b.add_override(k, v);
        }
        b.close().build()
    }
}

impl FromXML for ContentTypes {
    fn from_xml<R: Read>(reader: R) -> Result<Self, ReaderError> {
        let parser = EventReader::new(reader);
        let mut s = Self::default();
        let mut depth = 0;
        for e in parser {
            match e {
                Ok(XmlEvent::StartElement { attributes, .. }) => {
                    if depth == 1 {
                        let namespace = attributes[0].value.clone();
                        let path = attributes[1].value.clone();
                        s = s.add_content(path, namespace);
                    }
                    depth += 1;
                }
                Ok(XmlEvent::EndElement { .. }) => {
                    depth -= 1;
                }
                Err(_) => {}
                _ => {}
            }
        }
        Ok(s)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;

    #[test]
    fn test_from_xml() {
        let xml = r#"<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
        <Override ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml" PartName="/word/document.xml"></Override></Types>"#;
        let c = ContentTypes::from_xml(xml.as_bytes()).unwrap();
        let mut types = BTreeMap::new();
        types.insert(
            "/word/document.xml".to_owned(),
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"
                .to_owned(),
        );
        assert_eq!(
            ContentTypes {
                types,
                web_extension_count: 1,
                custom_xml_count: 1,
                header_count: 0,
                footer_count: 0,
            },
            c
        );
    }
}
