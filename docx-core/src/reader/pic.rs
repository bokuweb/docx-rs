#![allow(clippy::single_match)]

use std::io::Read;
use std::str::FromStr;

use xml::attribute::OwnedAttribute;
use xml::reader::{EventReader, XmlEvent};

use super::*;

impl ElementReader for Pic {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        _attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut pic = Pic::with_empty();
        loop {
            let e = r.next();
            match e {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    if let Ok(e) = AXMLElement::from_str(&name.local_name) {
                        match e {
                            AXMLElement::Blip => {
                                if let Some(id) = read(&attributes, "embed") {
                                    pic = pic.id(id)
                                }
                            }
                            AXMLElement::Off => {
                                let mut offset_x: i32 = 0;
                                let mut offset_y: i32 = 0;
                                if let Some(x) = read(&attributes, "x") {
                                    if let Ok(x) = f64::from_str(&x) {
                                        offset_x = x as i32;
                                    }
                                }
                                if let Some(y) = read(&attributes, "y") {
                                    if let Ok(y) = f64::from_str(&y) {
                                        offset_y = y as i32;
                                    }
                                }
                                pic = pic.offset_x(offset_x).offset_y(offset_y);
                            }
                            AXMLElement::Ext => {
                                let mut w: u32 = 0;
                                let mut h: u32 = 0;
                                if let Some(x) = read(&attributes, "cx") {
                                    if let Ok(x) = u32::from_str(&x) {
                                        w = x;
                                    }
                                }
                                if let Some(y) = read(&attributes, "cy") {
                                    if let Ok(y) = u32::from_str(&y) {
                                        h = y;
                                    }
                                }
                                pic = pic.size(w, h);
                            }
                            _ => {}
                        }
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    let e = PicXMLElement::from_str(&name.local_name).unwrap();
                    if e == PicXMLElement::Pic {
                        return Ok(pic);
                    }
                }
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
    }
}
