use std::io::Read;
use std::str::FromStr;

use xml::attribute::OwnedAttribute;
use xml::reader::{EventReader, XmlEvent};

use super::*;
use crate::types::*;

impl ElementReader for TableBorders {
    fn read<R: Read>(r: &mut EventReader<R>, _: &[OwnedAttribute]) -> Result<Self, ReaderError> {
        let mut borders = TableBorders::with_empty();
        loop {
            let e = r.next();
            match e {
                Ok(XmlEvent::StartElement {
                    attributes, name, ..
                }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    match e {
                        XMLElement::Top => {
                            let attr = read_border(&attributes)?;
                            let mut border = TableBorder::new(BorderPosition::Top)
                                .border_type(attr.border_type)
                                .color(attr.color);
                            if let Some(size) = attr.size {
                                border = border.size(size as usize);
                            };
                            borders = borders.set(border);
                            continue;
                        }
                        XMLElement::Right => {
                            let attr = read_border(&attributes)?;
                            let mut border = TableBorder::new(BorderPosition::Right)
                                .border_type(attr.border_type)
                                .color(attr.color);
                            if let Some(size) = attr.size {
                                border = border.size(size as usize);
                            };
                            borders = borders.set(border);
                            continue;
                        }
                        XMLElement::Bottom => {
                            let attr = read_border(&attributes)?;
                            let mut border = TableBorder::new(BorderPosition::Bottom)
                                .border_type(attr.border_type)
                                .color(attr.color);
                            if let Some(size) = attr.size {
                                border = border.size(size as usize);
                            };
                            borders = borders.set(border);
                            continue;
                        }
                        XMLElement::Left => {
                            let attr = read_border(&attributes)?;
                            let mut border = TableBorder::new(BorderPosition::Left)
                                .border_type(attr.border_type)
                                .color(attr.color);
                            if let Some(size) = attr.size {
                                border = border.size(size as usize);
                            };
                            borders = borders.set(border);
                            continue;
                        }
                        XMLElement::InsideH => {
                            let attr = read_border(&attributes)?;
                            let mut border = TableBorder::new(BorderPosition::InsideH)
                                .border_type(attr.border_type)
                                .color(attr.color);
                            if let Some(size) = attr.size {
                                border = border.size(size as usize);
                            };
                            borders = borders.set(border);
                            continue;
                        }
                        XMLElement::InsideV => {
                            let attr = read_border(&attributes)?;
                            let mut border = TableBorder::new(BorderPosition::InsideV)
                                .border_type(attr.border_type)
                                .color(attr.color);
                            if let Some(size) = attr.size {
                                border = border.size(size as usize);
                            };
                            borders = borders.set(border);
                            continue;
                        }
                        _ => {}
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    if e == XMLElement::TableBorders {
                        return Ok(borders);
                    }
                }
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
    }
}
