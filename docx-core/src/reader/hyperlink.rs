use std::io::Read;
use std::str::FromStr;

use xml::attribute::OwnedAttribute;
use xml::reader::{EventReader, XmlEvent};

use super::*;

use super::attributes::*;

impl ElementReader for Hyperlink {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut rid: Option<String> = read(attrs, "id");
        let mut anchor: Option<String> = read(attrs, "anchor");
        let history: Option<String> = read(attrs, "history");
        let mut link = Hyperlink {
            link: if anchor.is_some() {
                HyperlinkData::Anchor {
                    anchor: anchor.take().unwrap(),
                }
            } else {
                HyperlinkData::External {
                    rid: rid.take().unwrap_or_default(),
                    path: String::default(), // not used
                }
            },
            history: history.map(|h| usize::from_str(&h).unwrap_or(1)),
            children: vec![],
        };

        loop {
            let e = r.next();
            match e {
                Ok(XmlEvent::StartElement {
                    attributes, name, ..
                }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();

                    match e {
                        XMLElement::Run => {
                            if let Ok(run) = Run::read(r, attrs) {
                                link = link.add_run(run);
                            }
                            continue;
                        }
                        XMLElement::Insert => {
                            if let Ok(ins) = Insert::read(r, &attributes) {
                                link = link.add_insert(ins);
                            }
                            continue;
                        }
                        XMLElement::Delete => {
                            if let Ok(del) = Delete::read(r, &attributes) {
                                link = link.add_delete(del);
                            }
                            continue;
                        }
                        XMLElement::BookmarkStart => {
                            if let Ok(s) = BookmarkStart::read(r, &attributes) {
                                link = link.add_bookmark_start(s.id, s.name);
                            }
                            continue;
                        }
                        XMLElement::BookmarkEnd => {
                            if let Ok(e) = BookmarkEnd::read(r, &attributes) {
                                link = link.add_bookmark_end(e.id);
                            }
                            continue;
                        }
                        XMLElement::CommentRangeStart => {
                            if let Some(id) = read(&attributes, "id") {
                                if let Ok(id) = usize::from_str(&id) {
                                    let comment = Comment::new(id);
                                    link = link.add_comment_start(comment);
                                }
                            }
                            continue;
                        }
                        XMLElement::CommentRangeEnd => {
                            if let Some(id) = read(&attributes, "id") {
                                if let Ok(id) = usize::from_str(&id) {
                                    link = link.add_comment_end(id);
                                }
                            }
                            continue;
                        }
                        _ => {}
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    if e == XMLElement::Hyperlink {
                        return Ok(link);
                    }
                }
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
    }
}
