use std::io::Read;
use std::str::FromStr;

use xml::attribute::OwnedAttribute;
use xml::reader::{EventReader, XmlEvent};

use super::*;

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
