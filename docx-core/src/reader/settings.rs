use std::io::Read;
use xml::reader::{EventReader, XmlEvent};

use super::*;
use crate::reader::{FromXML, ReaderError};

use std::str::FromStr;

impl FromXML for Settings {
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
                    match e {
                        XMLElement::DocId => {
                            for a in attributes {
                                if let Some(prefix) = a.name.prefix {
                                    let local_name = &a.name.local_name;
                                    // Ignore w14:val
                                    if local_name == "val" && prefix == "w15" {
                                        settings = settings.doc_id(
                                            &a.value.to_owned().replace("{", "").replace("}", ""),
                                        );
                                    }
                                }
                            }
                        }
                        XMLElement::DocVar => {
                            let name = attributes::read_name(&attributes);
                            let val = attributes::read_val(&attributes);
                            if let Some(name) = name {
                                if let Some(val) = val {
                                    settings = settings.add_doc_var(name, val);
                                }
                            }
                        }
                        XMLElement::DefaultTabStop => {
                            let val = attributes::read_val(&attributes);
                            if let Some(val) = val {
                                if let Ok(val) = f32::from_str(&val) {
                                    settings = settings.default_tab_stop(val as usize);
                                }
                            }
                        }
                        _ => {}
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    if let XMLElement::Settings = e {
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
