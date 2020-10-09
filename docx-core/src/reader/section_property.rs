use std::io::Read;
use std::str::FromStr;

use xml::attribute::OwnedAttribute;
use xml::reader::{EventReader, XmlEvent};

use super::*;

fn read_page_size(attributes: &[OwnedAttribute]) -> Result<PageSize, ReaderError> {
    let mut size = PageSize::new();
    for a in attributes {
        let local_name = &a.name.local_name;
        match local_name.as_str() {
            "w" => {
                size = size.width(f32::from_str(&a.value)? as u32);
            }
            "h" => {
                size = size.height(f32::from_str(&a.value)? as u32);
            }
            _ => {}
        }
    }
    Ok(size)
}

fn read_page_margin(
    attributes: &[OwnedAttribute],
) -> Result<crate::types::PageMargin, ReaderError> {
    let mut margin = crate::types::PageMargin::new();
    for a in attributes {
        let local_name = &a.name.local_name;
        match local_name.as_str() {
            "top" => {
                margin = margin.top(f32::from_str(&a.value)? as u32);
            }
            "right" => {
                margin = margin.right(f32::from_str(&a.value)? as u32);
            }
            "bottom" => {
                margin = margin.bottom(f32::from_str(&a.value)? as u32);
            }
            "left" => {
                margin = margin.left(f32::from_str(&a.value)? as u32);
            }
            "header" => {
                margin = margin.header(f32::from_str(&a.value)? as u32);
            }
            "footer" => {
                margin = margin.footer(f32::from_str(&a.value)? as u32);
            }
            "gutter" => {
                margin = margin.gutter(f32::from_str(&a.value)? as u32);
            }
            _ => {}
        }
    }
    Ok(margin)
}

impl ElementReader for SectionProperty {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        _attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut sp = SectionProperty::new();
        loop {
            let e = r.next();
            match e {
                Ok(XmlEvent::StartElement {
                    attributes, name, ..
                }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    match e {
                        XMLElement::PageMargin => {
                            let margin = read_page_margin(&attributes)?;
                            sp = sp.page_margin(margin);
                        }
                        XMLElement::PageSize => {
                            let size = read_page_size(&attributes)?;
                            sp = sp.page_size(size);
                        }
                        _ => {}
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    if e == XMLElement::SectionProperty {
                        return Ok(sp);
                    }
                }
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
    }
}
