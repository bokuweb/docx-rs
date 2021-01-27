use std::io::Read;
use std::str::FromStr;

use xml::attribute::OwnedAttribute;
use xml::reader::{EventReader, XmlEvent};

use super::*;

impl ElementReader for Delete {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut del = Delete::new();
        loop {
            let e = r.next();
            match e {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    let e = XMLElement::from_str(&name.local_name)
                        .expect("should convert to XMLElement");
                    match e {
                        XMLElement::Run => {
                            del = del.add_run(Run::read(r, attrs)?);
                        }
                        XMLElement::CommentRangeStart => {
                            if let Some(id) = read(&attributes, "id") {
                                if let Ok(id) = usize::from_str(&id) {
                                    let comment = Comment::new(id);
                                    del = del.add_comment_start(comment);
                                }
                            }
                        }
                        XMLElement::CommentRangeEnd => {
                            if let Some(id) = read(&attributes, "id") {
                                if let Ok(id) = usize::from_str(&id) {
                                    del = del.add_comment_end(id);
                                }
                            }
                        }
                        _ => {}
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    if e == XMLElement::Delete {
                        for attr in attrs {
                            let local_name = &attr.name.local_name;
                            if local_name == "author" {
                                del = del.author(&attr.value);
                            } else if local_name == "date" {
                                del = del.date(&attr.value);
                            }
                        }
                        return Ok(del);
                    }
                }
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
    }
}
