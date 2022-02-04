use std::io::Read;
use std::str::FromStr;

use xml::attribute::OwnedAttribute;
use xml::reader::{EventReader, XmlEvent};

use crate::HeightRule;

use super::attributes::*;
use super::*;

impl ElementReader for TableRow {
    fn read<R: Read>(r: &mut EventReader<R>, _: &[OwnedAttribute]) -> Result<Self, ReaderError> {
        let mut cells = vec![];
        let mut grid_after = None;
        let mut width_after = None;
        let mut grid_before = None;
        let mut width_before = None;
        let mut row_height = None;
        let mut del = None;
        let mut ins = None;
        let mut height_rule = Some(HeightRule::AtLeast);
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
                        XMLElement::GridBefore => {
                            if let Some(v) = read_val(&attributes) {
                                grid_before = Some(u32::from_str(&v)?);
                            }
                        }
                        XMLElement::WidthBefore => {
                            if let Ok(v) = read_width(&attributes) {
                                width_before = Some(v.0 as f32);
                            }
                        }
                        XMLElement::HeightRule => {
                            if let Some(v) = read_val(&attributes) {
                                if let Ok(r) = HeightRule::from_str(&v) {
                                    height_rule = Some(r);
                                }
                            }
                        }
                        XMLElement::TableRowHeight => {
                            if let Some(v) = read_val(&attributes) {
                                let h = f32::from_str(&v);
                                if let Ok(h) = h {
                                    row_height = Some(h);
                                }
                            }
                        }
                        XMLElement::Delete => {
                            if let Ok(d) = Delete::read(r, &attributes) {
                                del = Some(d);
                            }
                        }
                        XMLElement::Insert => {
                            if let Ok(i) = Insert::read(r, &attributes) {
                                ins = Some(i);
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

                        if let Some(grid_before) = grid_before {
                            row = row.grid_before(grid_before);
                        }

                        if let Some(row_height) = row_height {
                            row = row.row_height(row_height);
                        }

                        if let Some(width_after) = width_after {
                            row = row.width_after(width_after);
                        }

                        if let Some(width_before) = width_before {
                            row = row.width_before(width_before);
                        }

                        if let Some(height_rule) = height_rule {
                            row = row.height_rule(height_rule);
                        }

                        if let Some(del) = del {
                            row = row.delete(del);
                        }

                        if let Some(ins) = ins {
                            row = row.insert(ins);
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
