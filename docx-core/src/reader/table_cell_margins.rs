use std::io::Read;
use std::str::FromStr;

use xml::attribute::OwnedAttribute;
use xml::reader::{EventReader, XmlEvent};

use super::*;

impl ElementReader for TableCellMargins {
    fn read<R: Read>(r: &mut EventReader<R>, _: &[OwnedAttribute]) -> Result<Self, ReaderError> {
        let mut margins = TableCellMargins::default();
        loop {
            let e = r.next();
            match e {
                Ok(XmlEvent::StartElement {
                    attributes, name, ..
                }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    match e {
                        XMLElement::Top => {
                            if let Ok(width) = read_width(&attributes) {
                                margins = margins.margin_top(width.0 as usize, width.1)
                            }
                        }
                        XMLElement::Right => {
                            if let Ok(width) = read_width(&attributes) {
                                margins = margins.margin_right(width.0 as usize, width.1)
                            }
                        }
                        XMLElement::Bottom => {
                            if let Ok(width) = read_width(&attributes) {
                                margins = margins.margin_bottom(width.0 as usize, width.1)
                            }
                        }
                        XMLElement::Left => {
                            if let Ok(width) = read_width(&attributes) {
                                margins = margins.margin_left(width.0 as usize, width.1)
                            }
                        }
                        _ => {}
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    if e == XMLElement::TableCellMargin {
                        return Ok(margins);
                    }
                }
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
    }
}
