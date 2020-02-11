use std::io::Read;
use xml::reader::{EventReader, XmlEvent};

use super::*;
use crate::reader::{FromXML, ReaderError};

use std::str::FromStr;

impl FromXML for Styles {
    fn from_xml<R: Read>(reader: R) -> Result<Self, ReaderError> {
        let mut parser = EventReader::new(reader);
        let mut styles = Self::default();
        loop {
            let e = parser.next();
            match e {
                Ok(XmlEvent::StartElement {
                    attributes, name, ..
                }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    if let XMLElement::Style = e {
                        let s = Style::read(&mut parser, &attributes)?;
                        styles = styles.add_style(s);
                        continue;
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    if let XMLElement::Styles = e {
                        break;
                    }
                }
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
        Ok(styles)
    }
}

// #[cfg(test)]
// mod tests {
//
//     use super::*;
//     #[cfg(test)]
//     use pretty_assertions::assert_eq;
//
//     #[test]
//     fn test_from_xml() {
//         let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
// <Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
//   <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties" Target="docProps/core.xml" />
// </Relationships>"#;
//         let c = Rels::from_xml(xml.as_bytes()).unwrap();
//         let mut rels = Vec::new();
//         rels.push((
//             "http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties"
//                 .to_owned(),
//             "rId1".to_owned(),
//             "docProps/core.xml".to_owned(),
//         ));
//         assert_eq!(Rels { rels }, c);
//     }
// }
//
