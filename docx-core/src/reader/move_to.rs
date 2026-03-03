use std::io::Read;
use std::str::FromStr;

use super::*;

impl ElementReader for MoveTo {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut mt = MoveTo::new_with_empty();
        loop {
            let e = r.next();
            match e {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    match e {
                        XMLElement::Run => mt = mt.add_run(Run::read(r, &attributes)?),
                        XMLElement::Delete => mt = mt.add_delete(Delete::read(r, &attributes)?),
                        XMLElement::CommentRangeStart => {
                            if let Some(id) = read(&attributes, "id") {
                                if let Ok(id) = usize::from_str(&id) {
                                    let comment = Comment::new(id);
                                    mt = mt.add_comment_start(comment);
                                }
                            }
                            continue;
                        }
                        XMLElement::CommentRangeEnd => {
                            if let Some(id) = read(&attributes, "id") {
                                if let Ok(id) = usize::from_str(&id) {
                                    mt = mt.add_comment_end(id);
                                }
                            }
                            continue;
                        }
                        XMLElement::MoveFrom => {
                            // Skip moveFrom subtree — ghost text must not flatten into MoveTo children
                            ignore::ignore_element(XMLElement::MoveFrom, XMLElement::MoveFrom, r);
                        }
                        _ => {}
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    if e == XMLElement::MoveTo {
                        for attr in attrs {
                            let local_name = &attr.name.local_name;
                            if local_name == "author" {
                                mt = mt.author(&attr.value);
                            } else if local_name == "date" {
                                mt = mt.date(&attr.value);
                            }
                        }
                        return Ok(mt);
                    }
                }
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
    }
}
