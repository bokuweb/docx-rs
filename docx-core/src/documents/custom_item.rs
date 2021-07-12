use crate::documents::BuildXML;
use crate::{ParseXmlError, XmlData, XmlDocument};
use serde::ser::{SerializeSeq, SerializeStruct};
use serde::Serialize;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct CustomItem(XmlDocument);

impl FromStr for CustomItem {
    type Err = ParseXmlError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(CustomItem(XmlDocument::from_str(s)?))
    }
}

impl Serialize for CustomItem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.0.data.len()))?;
        for e in self.0.data.iter() {
            seq.serialize_element(&CustomXmlData(e.clone()))?;
        }
        seq.end()
    }
}

#[derive(Debug, Clone)]
pub struct CustomXmlData(XmlData);

impl Serialize for CustomXmlData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut t = serializer.serialize_struct("XmlData", 4)?;
        t.serialize_field("name", &self.0.name)?;
        t.serialize_field("attributes", &self.0.attributes)?;
        t.serialize_field("data", &self.0.data)?;
        let sub_els: Vec<CustomXmlData> = self
            .0
            .sub_elements
            .iter()
            .map(|e| CustomXmlData(e.clone()))
            .collect();
        t.serialize_field("children", &sub_els)?;
        t.end()
    }
}

impl BuildXML for CustomItem {
    fn build(&self) -> Vec<u8> {
        self.0.to_string().as_bytes().to_vec()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;

    #[test]
    fn test_custom_xml() {
        let c = CustomItem::from_str(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>
        <ds:datastoreItem ds:itemID="{06AC5857-5C65-A94A-BCEC-37356A209BC3}"
            xmlns:ds="http://schemas.openxmlformats.org/officeDocument/2006/customXml">
            <ds:schemaRefs>
                <ds:schemaRef ds:uri="https://hoge.com"/>
            </ds:schemaRefs>
        </ds:datastoreItem>"#,
        )
        .unwrap();

        assert_eq!(
            c.0.to_string(),
        "<ds:datastoreItem ds:itemID=\"{06AC5857-5C65-A94A-BCEC-37356A209BC3}\" xmlns:ds=\"http://schemas.openxmlformats.org/officeDocument/2006/customXml\" xmlns:xml=\"http://www.w3.org/XML/1998/namespace\" xmlns:xmlns=\"http://www.w3.org/2000/xmlns/\">\n    <ds:schemaRefs>\n        <ds:schemaRef ds:uri=\"https://hoge.com\">\n        </ds:schemaRef>\n\n    </ds:schemaRefs>\n\n</ds:datastoreItem>\n"
        );
        assert_eq!(
            serde_json::to_string(&c).unwrap(),
        "[{\"name\":\"ds:datastoreItem\",\"attributes\":[[\"ds:itemID\",\"{06AC5857-5C65-A94A-BCEC-37356A209BC3}\"],[\"xmlns:ds\",\"http://schemas.openxmlformats.org/officeDocument/2006/customXml\"],[\"xmlns:xml\",\"http://www.w3.org/XML/1998/namespace\"],[\"xmlns:xmlns\",\"http://www.w3.org/2000/xmlns/\"]],\"data\":null,\"children\":[{\"name\":\"ds:schemaRefs\",\"attributes\":[],\"data\":null,\"children\":[{\"name\":\"ds:schemaRef\",\"attributes\":[[\"ds:uri\",\"https://hoge.com\"]],\"data\":null,\"children\":[]}]}]}]"
        );
    }
}
