use std::io::Read;
use std::str::FromStr;

use xml::attribute::OwnedAttribute;
use xml::reader::{EventReader, XmlEvent};

use super::*;
use crate::types::*;

impl ElementReader for TableCellProperty {
    fn read<R: Read>(r: &mut EventReader<R>, _: &[OwnedAttribute]) -> Result<Self, ReaderError> {
        let mut property = TableCellProperty::new();
        loop {
            let e = r.next();
            match e {
                Ok(XmlEvent::StartElement {
                    attributes, name, ..
                }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    ignore::ignore_element(e.clone(), XMLElement::TableCellPropertyChange, r);

                    match e {
                        XMLElement::TableCellWidth => {
                            let mut w = 0;
                            let mut width_type = WidthType::Auto;
                            for a in attributes {
                                let local_name = &a.name.local_name;
                                if local_name == "type" {
                                    width_type = WidthType::from_str(&a.value)?;
                                } else if local_name == "w" {
                                    w = usize::from_str(&a.value)?;
                                }
                            }
                            property = property.width(w, width_type);
                        }
                        XMLElement::TableGridSpan => {
                            if let Some(a) = &attributes.get(0) {
                                property = property.grid_span(usize::from_str(&a.value)?)
                            }
                        }
                        XMLElement::TableVMerge => {
                            if let Some(a) = &attributes.get(0) {
                                property = property.vertical_merge(VMergeType::from_str(&a.value)?);
                            } else {
                                // Treat as a continue without attribute
                                property = property.vertical_merge(VMergeType::Continue)
                            }
                        }
                        XMLElement::VAlign => {
                            if let Some(a) = &attributes.get(0) {
                                property = property.vertical_align(VAlignType::from_str(&a.value)?);
                            }
                        }
                        XMLElement::Shading => {
                            if let Ok(shd) = Shading::read(r, &attributes) {
                                property = property.shading(shd);
                            }
                        }
                        XMLElement::TextDirection => {
                            if let Some(a) = &attributes.get(0) {
                                if let Ok(v) = TextDirectionType::from_str(&a.value) {
                                    property = property.text_direction(v);
                                }
                            }
                        }
                        XMLElement::TableCellBorders => {
                            let borders = TableCellBorders::read(r, &attributes)?;
                            property = property.set_borders(borders);
                        }
                        _ => {}
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    if e == XMLElement::TableCellProperty {
                        return Ok(property);
                    }
                }
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
    }
}
