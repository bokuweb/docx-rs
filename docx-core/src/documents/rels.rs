use serde::{Deserialize, Serialize};
use std::io::Read;
use xml::reader::{EventReader, XmlEvent};

use crate::documents::BuildXML;
use crate::reader::{FromXML, ReaderError};
use crate::xml_builder::*;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Rels {
    rels: Vec<(String, String, String)>,
}

impl Rels {
    pub fn new() -> Rels {
        Default::default()
    }

    pub fn set_default(mut self) -> Self {
        self.rels.push((
            "http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties"
                .to_owned(),
            "rId1".to_owned(),
            "docProps/core.xml".to_owned(),
        ));
        self.rels.push(
            ("http://schemas.openxmlformats.org/officeDocument/2006/relationships/extended-properties".to_owned(),
            "rId2".to_owned(), "docProps/app.xml".to_owned()),
        );
        self.rels.push((
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument"
                .to_owned(),
            "rId3".to_owned(),
            "word/document.xml".to_owned(),
        ));
        self
    }

    pub fn add_rel(
        mut self,
        id: impl Into<String>,
        rel_type: impl Into<String>,
        target: impl Into<String>,
    ) -> Self {
        self.rels.push((rel_type.into(), id.into(), target.into()));
        self
    }

    pub fn find_target(&self, rel_type: &str) -> Option<&(String, String, String)> {
        self.rels.iter().find(|rel| rel.0 == rel_type)
    }
}

impl Default for Rels {
    fn default() -> Self {
        Rels { rels: Vec::new() }
    }
}

impl BuildXML for Rels {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        let mut b = b
            .declaration(None)
            .open_relationships("http://schemas.openxmlformats.org/package/2006/relationships");
        for (k, id, v) in self.rels.iter() {
            b = b.relationship(id, k, v);
        }
        b.close().build()
    }
}

impl FromXML for Rels {
    fn from_xml<R: Read>(reader: R) -> Result<Self, ReaderError> {
        let parser = EventReader::new(reader);
        let mut s = Self::default();
        let mut depth = 0;
        for e in parser {
            match e {
                Ok(XmlEvent::StartElement { attributes, .. }) => {
                    if depth == 1 {
                        let mut id = "".to_owned();
                        let mut rel_type = "".to_owned();
                        let mut target = "".to_owned();
                        for attr in attributes {
                            let name: &str = &attr.name.local_name;
                            if name == "Id" {
                                id = attr.value.clone();
                            } else if name == "Type" {
                                rel_type = attr.value.clone();
                            } else if name == "Target" {
                                target = attr.value.clone();
                            }
                        }
                        s = s.add_rel(id, rel_type, target);
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
    use std::str;

    #[test]
    fn test_build() {
        let c = Rels::new().set_default();
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<?xml version="1.0" encoding="UTF-8"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties" Target="docProps/core.xml" />
  <Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/extended-properties" Target="docProps/app.xml" />
  <Relationship Id="rId3" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="word/document.xml" />
</Relationships>"#
        );
    }

    #[test]
    fn test_from_xml() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties" Target="docProps/core.xml" />
</Relationships>"#;
        let c = Rels::from_xml(xml.as_bytes()).unwrap();
        let mut rels = Vec::new();
        rels.push((
            "http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties"
                .to_owned(),
            "rId1".to_owned(),
            "docProps/core.xml".to_owned(),
        ));
        assert_eq!(Rels { rels }, c);
    }
}
