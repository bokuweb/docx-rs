use std::io::Read;
use std::str::FromStr;

use xml::attribute::OwnedAttribute;
use xml::reader::{EventReader, XmlEvent};

use crate::types::*;

use super::*;

fn read_custom_tab_stop_type(attributes: &[OwnedAttribute]) -> Result<TabValueType, ReaderError> {
    for a in attributes {
        let local_name = &a.name.local_name;
        if local_name == "val" {
            let v = a.value.to_owned();
            if let Ok(t) = TabValueType::from_str(&v) {
                return Ok(t);
            }
        }
    }
    Err(ReaderError::TypeError(crate::TypeError::FromStrError))
}

fn read_custom_tab_stop_leader(
    attributes: &[OwnedAttribute],
) -> Result<TabLeaderType, ReaderError> {
    for a in attributes {
        let local_name = &a.name.local_name;
        if local_name == "leader" {
            let v = a.value.to_owned();
            if let Ok(t) = TabLeaderType::from_str(&v) {
                return Ok(t);
            }
        }
    }
    Err(ReaderError::TypeError(crate::TypeError::FromStrError))
}

fn read_custom_tab_stop_pos(attributes: &[OwnedAttribute]) -> Result<f32, ReaderError> {
    for a in attributes {
        let local_name = &a.name.local_name;
        if local_name == "pos" {
            let v = a.value.to_owned();
            if let Ok(t) = f32::from_str(&v) {
                return Ok(t);
            }
        }
    }
    Err(ReaderError::TypeError(crate::TypeError::FromStrError))
}

impl ElementReader for Tab {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut tab = Tab::new();
        if let Ok(t) = read_custom_tab_stop_type(attrs) {
            tab = tab.val(t);
        }
        if let Ok(pos) = read_custom_tab_stop_pos(attrs) {
            tab = tab.pos(pos as usize);
        }
        if let Ok(leader) = read_custom_tab_stop_leader(attrs) {
            tab = tab.leader(leader);
        }
        loop {
            let e = r.next();
            match e {
                Ok(XmlEvent::EndElement { name, .. }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    if e == XMLElement::Tab {
                        return Ok(tab);
                    }
                }
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
    }
}
