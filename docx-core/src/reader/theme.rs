use std::io::Read;
use std::str::FromStr;

use crate::reader::*;
use xml::reader::{EventReader, XmlEvent};

impl FromXML for Theme {
    fn from_xml<R: Read>(reader: R) -> Result<Self, ReaderError> {
        let mut parser = EventReader::new(reader);
        let mut theme = Self::default();
        loop {
            let e = parser.next();
            match e {
                Ok(XmlEvent::StartElement {
                    attributes, name, ..
                }) => {
                    let e = AXMLElement::from_str(&name.local_name).unwrap();
                    #[allow(clippy::single_match)]
                    match e {
                        AXMLElement::FontScheme => {
                            if let Ok(f) = FontScheme::read(&mut parser, &attributes) {
                                theme.font_schema = f;
                            }
                            continue;
                        }
                        _ => {}
                    }
                }
                Ok(XmlEvent::EndDocument) => break,
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
        Ok(theme)
    }
}
