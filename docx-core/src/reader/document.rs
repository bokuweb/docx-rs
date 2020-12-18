use std::io::Read;
use std::str::FromStr;

use crate::reader::*;
use xml::reader::{EventReader, XmlEvent};

use super::{Paragraph, Table};

impl FromXML for Document {
    fn from_xml<R: Read>(reader: R) -> Result<Self, ReaderError> {
        let mut parser = EventReader::new(reader);
        let mut doc = Self::default();
        loop {
            let e = parser.next();
            match e {
                Ok(XmlEvent::StartElement {
                    attributes, name, ..
                }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    match e {
                        XMLElement::Paragraph => {
                            let p = Paragraph::read(&mut parser, &attributes)?;
                            doc = doc.add_paragraph(p);
                            continue;
                        }
                        XMLElement::Table => {
                            let t = Table::read(&mut parser, &attributes)?;
                            doc = doc.add_table(t);
                            continue;
                        }
                        XMLElement::BookmarkStart => {
                            let s = BookmarkStart::read(&mut parser, &attributes)?;
                            doc = doc.add_bookmark_start(s.id, s.name);
                            continue;
                        }
                        XMLElement::BookmarkEnd => {
                            let e = BookmarkEnd::read(&mut parser, &attributes)?;
                            doc = doc.add_bookmark_end(e.id);
                            continue;
                        }
                        XMLElement::CommentRangeStart => {
                            if let Some(id) = read(&attributes, "id") {
                                if let Ok(id) = usize::from_str(&id) {
                                    let comment = Comment::new(id);
                                    doc = doc.add_comment_start(comment);
                                }
                            }
                            continue;
                        }
                        XMLElement::CommentRangeEnd => {
                            if let Some(id) = read(&attributes, "id") {
                                if let Ok(id) = usize::from_str(&id) {
                                    doc = doc.add_comment_end(id);
                                }
                            }
                            continue;
                        }
                        XMLElement::SectionProperty => {
                            let e = SectionProperty::read(&mut parser, &attributes)?;
                            doc = doc.default_section_property(e);
                            continue;
                        }
                        _ => {}
                    }
                }
                Ok(XmlEvent::EndDocument) => break,
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
        Ok(doc)
    }
}
