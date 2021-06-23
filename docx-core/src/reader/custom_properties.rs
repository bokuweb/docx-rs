use std::io::Read;
use std::str::FromStr;

use xml::reader::{EventReader, XmlEvent};

use super::*;

impl FromXML for CustomProps {
    fn from_xml<R: Read>(reader: R) -> Result<Self, ReaderError> {
        let mut r = EventReader::new(reader);
        // TODO: Fow now, support only string.
        let mut props = CustomProps::new();
        loop {
            let e = r.next();
            match e {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    if let Ok(XMLElement::Property) = XMLElement::from_str(&name.local_name) {
                        if let Some(key) = read_name(&attributes) {
                            loop {
                                let e = r.next();
                                match e {
                                    Ok(XmlEvent::StartElement { name, .. }) => {
                                        // TODO: Fow now, support only string.
                                        if let Ok(VtXMLElement::Lpwstr) =
                                            VtXMLElement::from_str(&name.local_name)
                                        {
                                            let e = r.next();
                                            if let Ok(XmlEvent::Characters(c)) = e {
                                                props = props.add_custom_property(&key, c)
                                            }
                                        }
                                    }
                                    Ok(XmlEvent::EndElement { name, .. }) => {
                                        if let Ok(XMLElement::Property) =
                                            XMLElement::from_str(&name.local_name)
                                        {
                                            break;
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }
                Ok(XmlEvent::EndDocument { .. }) => {
                    return Ok(props);
                }
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
    }
}
