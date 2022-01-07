use std::io::Read;
use std::str::FromStr;

use xml::attribute::OwnedAttribute;
use xml::reader::{EventReader, XmlEvent};

use super::*;

use super::attributes::*;

impl ElementReader for StructuredDataTag {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut sdt = StructuredDataTag::new();
        loop {
            let e = r.next();
            match e {
                Ok(XmlEvent::StartElement {
                    attributes, name, ..
                }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();

                    match e {
                        XMLElement::Paragraph => {
                            if let Ok(p) = Paragraph::read(r, &attributes) {
                                sdt.children
                                    .push(StructuredDataTagChild::Paragraph(Box::new(p)));
                            }
                            continue;
                        }
                        XMLElement::Table => {
                            if let Ok(t) = Table::read(r, &attributes) {
                                sdt.children
                                    .push(StructuredDataTagChild::Table(Box::new(t)));
                            }
                            continue;
                        }
                        XMLElement::BookmarkStart => {
                            if let Ok(s) = BookmarkStart::read(r, &attributes) {
                                sdt.children.push(StructuredDataTagChild::BookmarkStart(s));
                            }
                            continue;
                        }
                        XMLElement::BookmarkEnd => {
                            if let Ok(e) = BookmarkEnd::read(r, &attributes) {
                                sdt.children.push(StructuredDataTagChild::BookmarkEnd(e));
                            }
                            continue;
                        }
                        XMLElement::CommentRangeStart => {
                            if let Some(id) = read(&attributes, "id") {
                                if let Ok(id) = usize::from_str(&id) {
                                    let comment = Comment::new(id);
                                    sdt.children.push(StructuredDataTagChild::CommentStart(
                                        Box::new(CommentRangeStart::new(comment)),
                                    ));
                                }
                            }
                            continue;
                        }
                        XMLElement::CommentRangeEnd => {
                            if let Some(id) = read(&attributes, "id") {
                                if let Ok(id) = usize::from_str(&id) {
                                    sdt.children.push(StructuredDataTagChild::CommentEnd(
                                        CommentRangeEnd::new(id),
                                    ));
                                }
                            }
                            continue;
                        }
                        XMLElement::Run => {
                            if let Ok(run) = Run::read(r, attrs) {
                                sdt.children.push(StructuredDataTagChild::Run(Box::new(run)));
                            }
                            continue;
                        }
                        _ => {}
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    if e == XMLElement::Paragraph {
                        return Ok(sdt);
                    }
                }
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
    }
}
