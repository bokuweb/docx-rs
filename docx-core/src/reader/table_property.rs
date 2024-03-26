use std::io::Read;
use std::str::FromStr;

use xml::attribute::OwnedAttribute;
use xml::reader::{EventReader, XmlEvent};

use crate::TableAlignmentType;

use super::*;

// TODO: layout: Option<TableLayout>,
impl ElementReader for TableProperty {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        _attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut tp = TableProperty::new();
        tp = tp.set_borders(TableBorders::with_empty());
        loop {
            let e = r.next();
            match e {
                Ok(XmlEvent::StartElement {
                    attributes, name, ..
                }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    match e {
                        XMLElement::TableBorders => {
                            if let Ok(borders) = TableBorders::read(r, &attributes) {
                                tp = tp.set_borders(borders);
                            }
                        }
                        XMLElement::TableCellMargin => {
                            if let Ok(margins) = TableCellMargins::read(r, &attributes) {
                                tp = tp.set_margins(margins);
                            }
                        }
                        XMLElement::TableWidth => {
                            if let Ok((w, width_type)) = read_width(&attributes) {
                                tp = tp.width(w as usize, width_type);
                            }
                        }
                        XMLElement::Justification => {
                            if let Ok(v) = TableAlignmentType::from_str(&attributes[0].value) {
                                tp = tp.align(v);
                            }
                        }
                        XMLElement::TableIndent => {
                            if let Ok((w, _)) = read_width(&attributes) {
                                if w != 0 {
                                    tp = tp.indent(w as i32);
                                }
                            }
                        }
                        XMLElement::TableStyle => {
                            if let Some(s) = read_val(&attributes) {
                                tp = tp.style(s);
                            }
                        }
                        XMLElement::TablePositionProperty => {
                            if let Ok(p) = TablePositionProperty::read(r, &attributes) {
                                tp = tp.position(p);
                            }
                        }
                        _ => {}
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    if e == XMLElement::TableProperty {
                        return Ok(tp);
                    }
                }
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
    }
}
