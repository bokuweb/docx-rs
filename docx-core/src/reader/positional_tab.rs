use std::io::Read;
use std::str::FromStr;

use xml::attribute::OwnedAttribute;
use xml::reader::{EventReader, XmlEvent};

use crate::{PositionalTabAlignmentType, PositionalTabRelativeTo};

use super::*;

fn read_leader(attributes: &[OwnedAttribute]) -> Result<TabLeaderType, ReaderError> {
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

fn read_alignment(
    attributes: &[OwnedAttribute],
) -> Result<PositionalTabAlignmentType, ReaderError> {
    for a in attributes {
        let local_name = &a.name.local_name;
        if local_name == "alignment" {
            let v = a.value.to_owned();
            if let Ok(t) = PositionalTabAlignmentType::from_str(&v) {
                return Ok(t);
            }
        }
    }
    Err(ReaderError::TypeError(crate::TypeError::FromStrError))
}

fn read_relative_to(attributes: &[OwnedAttribute]) -> Result<PositionalTabRelativeTo, ReaderError> {
    for a in attributes {
        let local_name = &a.name.local_name;
        if local_name == "alignment" {
            let v = a.value.to_owned();
            if let Ok(t) = PositionalTabRelativeTo::from_str(&v) {
                return Ok(t);
            }
        }
    }
    Err(ReaderError::TypeError(crate::TypeError::FromStrError))
}

impl ElementReader for PositionalTab {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut tab = PositionalTab::default();
        if let Ok(t) = read_alignment(attrs) {
            tab = tab.alignment(t);
        }
        if let Ok(v) = read_relative_to(attrs) {
            tab = tab.relative_to(v);
        }
        if let Ok(leader) = read_leader(attrs) {
            tab = tab.leader(leader);
        }
        loop {
            let e = r.next();
            match e {
                Ok(XmlEvent::EndElement { name, .. }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    if e == XMLElement::PTab {
                        return Ok(tab);
                    }
                }
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
    }
}
