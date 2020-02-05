use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Read;
use xml::reader::{EventReader, XmlEvent};

use crate::documents::BuildXML;
use crate::reader::{FromXML, ReaderError};
use crate::xml_builder::*;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ContentTypes {
    types: HashMap<String, String>,
}

impl ContentTypes {
    pub fn new() -> ContentTypes {
        Default::default()
    }

    pub fn add_content(mut self, namespace: impl Into<String>, path: impl Into<String>) -> Self {
        self.types.insert(namespace.into(), path.into());
        self
    }

    pub fn set_default(mut self) -> ContentTypes {
        self.types.insert(
            "application/vnd.openxmlformats-package.relationships+xml".to_owned(),
            "/_rels/.rels".to_owned(),
        );
        self.types.insert(
            "application/vnd.openxmlformats-officedocument.extended-properties+xml".to_owned(),
            "/docProps/app.xml".to_owned(),
        );
        self.types.insert(
            "application/vnd.openxmlformats-package.core-properties+xml".to_owned(),
            "/docProps/core.xml".to_owned(),
        );
        self.types.insert(
            "application/vnd.openxmlformats-package.relationships+xml".to_owned(),
            "/word/_rels/document.xml.rels".to_owned(),
        );
        self.types.insert(
            "application/vnd.openxmlformats-officedocument.wordprocessingml.settings+xml"
                .to_owned(),
            "/word/settings.xml".to_owned(),
        );
        self.types.insert(
            "application/vnd.openxmlformats-officedocument.wordprocessingml.fontTable+xml"
                .to_owned(),
            "/word/fontTable.xml".to_owned(),
        );
        self.types.insert(
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"
                .to_owned(),
            "/word/document.xml".to_owned(),
        );
        self.types.insert(
            "application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml".to_owned(),
            "/word/styles.xml".to_owned(),
        );
        self.types.insert(
            "application/vnd.openxmlformats-officedocument.wordprocessingml.comments+xml"
                .to_owned(),
            "/word/comments.xml".to_owned(),
        );
        self.types.insert(
            "application/vnd.openxmlformats-officedocument.wordprocessingml.numbering+xml"
                .to_owned(),
            "/word/numbering.xml".to_owned(),
        );
        self
    }
}

impl Default for ContentTypes {
    fn default() -> Self {
        ContentTypes {
            types: HashMap::new(),
        }
    }
}

impl BuildXML for ContentTypes {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        let mut b = b
            .declaration(None)
            .open_types("http://schemas.openxmlformats.org/package/2006/content-types");
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
                        s = s.add_content(namespace, path);
                    }
                    depth += 1;
                }
                Ok(XmlEvent::EndElement { .. }) => {
                    depth -= 1;
                }
                Err(_) => return Err(ReaderError::XMLReadError),
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
        let mut types = HashMap::new();
        types.insert(
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"
                .to_owned(),
            "/word/document.xml".to_owned(),
        );
        assert_eq!(ContentTypes { types }, c);
    }
}
