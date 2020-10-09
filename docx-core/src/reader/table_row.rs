use std::io::Read;
use std::str::FromStr;

use xml::attribute::OwnedAttribute;
use xml::reader::{EventReader, XmlEvent};

use super::attributes::*;
use super::*;

impl ElementReader for TableRow {
    fn read<R: Read>(r: &mut EventReader<R>, _: &[OwnedAttribute]) -> Result<Self, ReaderError> {
        let mut cells = vec![];
        let mut grid_after = None;
        let mut width_after = None;
        let mut row_height = None;
        loop {
            let e = r.next();
            match e {
                Ok(XmlEvent::StartElement {
                    attributes, name, ..
                }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();

                    ignore::ignore_element(e.clone(), XMLElement::TableRowPropertyChange, r);

                    match e {
                        XMLElement::TableCell => {
                            cells.push(TableCell::read(r, &attributes)?);
                            continue;
                        }
                        XMLElement::GridAfter => {
                            if let Some(v) = read_val(&attributes) {
                                grid_after = Some(u32::from_str(&v)?);
                            }
                        }
                        XMLElement::WidthAfter => {
                            if let Ok(v) = read_width(&attributes) {
                                width_after = Some(v.0 as f32);
                            }
                        }
                        XMLElement::TableRowHeight => {
                            if let Some(v) = read_val(&attributes) {
                                row_height = Some(f32::from_str(&v)?);
                            }
                        }
                        _ => {}
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    if e == XMLElement::TableRow {
                        let mut row = TableRow::new(cells);
                        if let Some(grid_after) = grid_after {
                            row = row.grid_after(grid_after);
                        }

                        if let Some(row_height) = row_height {
                            row = row.row_height(row_height);
                        }

                        if let Some(width_after) = width_after {
                            row = row.width_after(width_after);
                        }
                        return Ok(row);
                    }
                }
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
    }
}
