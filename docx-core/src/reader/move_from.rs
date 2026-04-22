use std::io::Read;
use std::str::FromStr;

use super::*;

impl ElementReader for MoveFrom {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut mf = MoveFrom::new();
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
                            mf = mf.add_run(Run::read(r, &attributes)?);
                        }
                        XMLElement::CommentRangeStart => {
                            if let Some(id) = read(&attributes, "id") {
                                if let Ok(id) = usize::from_str(&id) {
                                    let comment = Comment::new(id);
                                    mf = mf.add_comment_start(comment);
                                }
                            }
                        }
                        XMLElement::CommentRangeEnd => {
                            if let Some(id) = read(&attributes, "id") {
                                if let Ok(id) = usize::from_str(&id) {
                                    mf = mf.add_comment_end(id);
                                }
                            }
                        }
                        _ => {}
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    if e == XMLElement::MoveFrom {
                        for attr in attrs {
                            let local_name = &attr.name.local_name;
                            if local_name == "author" {
                                mf = mf.author(&attr.value);
                            } else if local_name == "date" {
                                mf = mf.date(&attr.value);
                            }
                        }
                        return Ok(mf);
                    }
                }
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
    }
}
