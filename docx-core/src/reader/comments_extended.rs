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
                        comments_extended.push(CommentExtended::read(&mut r, &attributes)?);
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
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
    }
}
