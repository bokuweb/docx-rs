use std::io::Read;
use std::str::FromStr;

use xml::reader::{EventReader, XmlEvent};

use super::*;

impl FromXML for CommentsExtended {
    fn from_xml<R: Read>(reader: R) -> Result<Self, ReaderError> {
        let mut r = EventReader::new(reader);
        let mut comments_extended: Vec<CommentExtended> = vec![];
        loop {
            let e = r.next();
            match e {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    let e = XMLElement::from_str(&name.local_name)
                        .expect("should convert to XMLElement");
                    if let XMLElement::CommentExtended = e {
                        if let Ok(ex) = CommentExtended::read(&mut r, &attributes) {
                            if let Some(pos) = comments_extended
                                .iter()
                                .position(|e| e.paragraph_id == ex.paragraph_id)
                            {
                                comments_extended[pos] = ex;
                            } else {
                                comments_extended.push(ex);
                            }
                        }
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    if e == XMLElement::CommentsExtended {
                        return Ok(CommentsExtended {
                            children: comments_extended,
                        });
                    }
                }
                Ok(XmlEvent::EndDocument { .. }) => {
                    return Ok(CommentsExtended {
                        children: comments_extended,
                    });
                }
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
    }
}
