#![allow(clippy::single_match)]

use std::io::Read;
use std::str::FromStr;

use xml::attribute::OwnedAttribute;
use xml::reader::{EventReader, XmlEvent};

use super::*;

fn read_typeface(attributes: &[OwnedAttribute]) -> Option<String> {
    for a in attributes {
        let local_name = &a.name.local_name;
        if let "typeface" = local_name.as_str() {
            return Some(a.value.to_string());
        }
    }
    None
}

fn read_script_and_typeface(attributes: &[OwnedAttribute]) -> Option<(String, String)> {
    let mut script = None;
    let mut typeface = None;
    for a in attributes {
        let local_name = &a.name.local_name;
        if let "script" = local_name.as_str() {
            script = Some(a.value.to_string());
        }
        if let "typeface" = local_name.as_str() {
            typeface = Some(a.value.to_string());
        }
    }
    if let (Some(script), Some(typeface)) = (script, typeface) {
        return Some((script, typeface));
    }
    None
}

impl ElementReader for FontGroup {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        _attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut f = FontGroup::default();
        loop {
            let e = r.next();
            match e {
                Ok(XmlEvent::StartElement {
                    attributes, name, ..
                }) => {
                    let e = AXMLElement::from_str(&name.local_name).unwrap();
                    match e {
                        AXMLElement::Latin => {
                            if let Some(t) = read_typeface(&attributes) {
                                f.latin = t;
                            }
                        }
                        AXMLElement::Ea => {
                            if let Some(t) = read_typeface(&attributes) {
                                f.ea = t;
                            }
                        }
                        AXMLElement::Cs => {
                            if let Some(t) = read_typeface(&attributes) {
                                f.cs = t;
                            }
                        }
                        AXMLElement::Font => {
                            if let Some((script, typeface)) = read_script_and_typeface(&attributes)
                            {
                                f.fonts.push(FontSchemeFont { script, typeface })
                            }
                        }
                        _ => {}
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    let e = AXMLElement::from_str(&name.local_name).unwrap();
                    match e {
                        AXMLElement::MajorFont | AXMLElement::MinorFont => {
                            return Ok(f);
                        }
                        _ => {}
                    }
                }
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
    }
}
