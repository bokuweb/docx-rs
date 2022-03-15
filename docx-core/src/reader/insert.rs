use std::io::Read;
use std::str::FromStr;

use xml::attribute::OwnedAttribute;
use xml::reader::{EventReader, XmlEvent};

use super::*;

impl ElementReader for Insert {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut ins = Insert::new_with_empty();
        loop {
            let e = r.next();
            match e {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    match e {
                        XMLElement::Run => ins = ins.add_run(Run::read(r, &attributes)?),
                        XMLElement::Delete => ins = ins.add_delete(Delete::read(r, &attributes)?),
                        XMLElement::CommentRangeStart => {
                            if let Some(id) = read(&attributes, "id") {
                                if let Ok(id) = usize::from_str(&id) {
                                    let comment = Comment::new(id);
                                    ins = ins.add_comment_start(comment);
                                }
                            }
                            continue;
                        }
                        XMLElement::CommentRangeEnd => {
                            if let Some(id) = read(&attributes, "id") {
                                if let Ok(id) = usize::from_str(&id) {
                                    ins = ins.add_comment_end(id);
                                }
                            }
                            continue;
                        }
                        _ => {}
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    if e == XMLElement::Insert {
                        for attr in attrs {
                            let local_name = &attr.name.local_name;
                            if local_name == "author" {
                                ins = ins.author(&attr.value);
                            } else if local_name == "date" {
                                ins = ins.date(&attr.value);
                            }
                        }
                        return Ok(ins);
                    }
                }
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
    }
}
