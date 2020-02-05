use std::io::Read;
use std::str::FromStr;

use crate::documents::BuildXML;
use crate::reader::*;
use crate::xml_builder::*;
use xml::reader::{EventReader, XmlEvent};

use super::{Paragraph, SectionProperty, Table};

#[derive(Debug, Clone)]
pub enum DocumentReaderState {
    Idle,
    Ready,
    Paragraph,
}

impl FromXML for Document {
    fn from_xml<R: Read>(reader: R) -> Result<Self, ReaderError> {
        let mut parser = EventReader::new(reader);
        let mut doc = Self::default();
        let mut depth = 0;
        let mut state = DocumentReaderState::Idle;

        loop {
            let e = parser.next();
            match e {
                Ok(XmlEvent::StartElement {
                    attributes, name, ..
                }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    match e {
                        XMLElement::Body => {
                            state = DocumentReaderState::Ready;
                            continue;
                        }
                        XMLElement::Paragraph => {
                            let p = Paragraph::read(&mut parser)?;
                            doc = doc.add_paragraph(p);
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
        // dbg!(&doc);
        Ok(doc)
    }
}
