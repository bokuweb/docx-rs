use std::io::Read;
use std::str::FromStr;

use xml::attribute::OwnedAttribute;
use xml::reader::{EventReader, XmlEvent};

use super::*;

impl ElementReader for Comment {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let id = usize::from_str(&read(attrs, "id").expect("should comment id exists."))?;
        let mut comment = Comment::new(id);
        if let Some(author) = read(attrs, "author") {
            comment = comment.author(author);
        };
        if let Some(date) = read(attrs, "date") {
            comment = comment.date(date);
        }
        loop {
            let e = r.next();
            match e {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    let e = XMLElement::from_str(&name.local_name)
                        .expect("should convert to XMLElement");
                    if let XMLElement::Paragraph = e {
                        let p = Paragraph::read(r, &attributes)?;
                        comment = comment.add_paragraph(p);
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    if e == XMLElement::Comment {
                        return Ok(comment);
                    }
                }
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
    }
}
