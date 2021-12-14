use crate::documents::BuildXML;
use crate::{ParseXmlError, XmlDocument};
use serde::ser::SerializeSeq;
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
            seq.serialize_element(e)?;
        }
        seq.end()
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
            r#"<ds:datastoreItem ds:itemID="{06AC5857-5C65-A94A-BCEC-37356A209BC3}" xmlns:ds="http://schemas.openxmlformats.org/officeDocument/2006/customXml">
<ds:schemaRefs>
<ds:schemaRef ds:uri="https://hoge.com"></ds:schemaRef></ds:schemaRefs></ds:datastoreItem>"#,
        )
        .unwrap();

        assert_eq!(
            c.0.to_string(),
            r#"<ds:datastoreItem ds:itemID="{06AC5857-5C65-A94A-BCEC-37356A209BC3}" xmlns:ds="http://schemas.openxmlformats.org/officeDocument/2006/customXml">
<ds:schemaRefs>
<ds:schemaRef ds:uri="https://hoge.com"></ds:schemaRef></ds:schemaRefs></ds:datastoreItem>"#
        );
        assert_eq!(
            serde_json::to_string(&c).unwrap(),
        "[{\"name\":\"ds:datastoreItem\",\"attributes\":[[\"ds:itemID\",\"{06AC5857-5C65-A94A-BCEC-37356A209BC3}\"],[\"xmlns:ds\",\"http://schemas.openxmlformats.org/officeDocument/2006/customXml\"]],\"data\":null,\"children\":[{\"name\":\"ds:schemaRefs\",\"attributes\":[],\"data\":null,\"children\":[{\"name\":\"ds:schemaRef\",\"attributes\":[[\"ds:uri\",\"https://hoge.com\"]],\"data\":null,\"children\":[]}]}]}]"
        );
    }
}
