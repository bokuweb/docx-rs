use std::io::Read;
use xml::reader::{EventReader, XmlEvent};

use super::*;
use crate::reader::{FromXML, ReaderError};

use std::str::FromStr;

impl FromXML for WebSettings {
    fn from_xml<R: Read>(reader: R) -> Result<Self, ReaderError> {
        let mut parser = EventReader::new(reader);
        let mut settings = Self::default();
        loop {
            let e = parser.next();
            match e {
                Ok(XmlEvent::StartElement {
                    attributes, name, ..
                }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    if let XMLElement::Div = e {
                        if let Ok(div) = Div::read(&mut parser, &attributes) {
                            settings.divs.push(div);
                        }
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    if let XMLElement::WebSettings = e {
                        break;
                    }
                }
                Ok(XmlEvent::EndDocument { .. }) => break,
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
        Ok(settings)
    }
}
