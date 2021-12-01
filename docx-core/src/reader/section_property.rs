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
                size = size.width(value_to_dax(&a.value)? as u32);
            }
            "h" => {
                size = size.height(value_to_dax(&a.value)? as u32);
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
                margin = margin.top(value_to_dax(&a.value)? as u32);
            }
            "right" => {
                margin = margin.right(value_to_dax(&a.value)? as u32);
            }
            "bottom" => {
                margin = margin.bottom(value_to_dax(&a.value)? as u32);
            }
            "left" => {
                margin = margin.left(value_to_dax(&a.value)? as u32);
            }
            "header" => {
                margin = margin.header(value_to_dax(&a.value)? as u32);
            }
            "footer" => {
                margin = margin.footer(value_to_dax(&a.value)? as u32);
            }
            "gutter" => {
                margin = margin.gutter(value_to_dax(&a.value)? as u32);
            }
            _ => {}
        }
    }
    Ok(margin)
}

fn read_header_or_footer_reference(
    attributes: &[OwnedAttribute],
) -> Result<(String, String), ReaderError> {
    let mut rid = "".to_owned();
    let mut header_type = "default".to_owned();
    for a in attributes {
        let local_name = &a.name.local_name;
        match local_name.as_str() {
            "type" => {
                header_type = a.value.to_owned();
            }
            "id" => {
                rid = a.value.to_owned();
            }
            _ => {}
        }
    }
    Ok((rid, header_type))
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
                        XMLElement::DocGrid => {
                            if let Ok(doc_grid) = DocGrid::read(r, &attributes) {
                                sp = sp.doc_grid(doc_grid);
                            }
                        }
                        XMLElement::HeaderReference => {
                            if let Ok((rid, header_type)) =
                                read_header_or_footer_reference(&attributes)
                            {
                                match header_type.as_str() {
                                    "default" => {
                                        sp.header_reference =
                                            Some(HeaderReference::new(header_type, rid));
                                    }
                                    "first" => {
                                        sp.first_header_reference =
                                            Some(HeaderReference::new(header_type, rid));
                                    }
                                    "even" => {
                                        sp.even_header_reference =
                                            Some(HeaderReference::new(header_type, rid));
                                    }
                                    _ => {}
                                }
                            }
                        }
                        XMLElement::FooterReference => {
                            if let Ok((rid, footer_type)) =
                                read_header_or_footer_reference(&attributes)
                            {
                                match footer_type.as_str() {
                                    "default" => {
                                        sp.footer_reference =
                                            Some(FooterReference::new(footer_type, rid));
                                    }
                                    "first" => {
                                        sp.first_footer_reference =
                                            Some(FooterReference::new(footer_type, rid));
                                    }
                                    "even" => {
                                        sp.even_footer_reference =
                                            Some(FooterReference::new(footer_type, rid));
                                    }
                                    _ => {}
                                }
                            }
                        }
                        XMLElement::TitlePg => sp = sp.title_pg(),
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
