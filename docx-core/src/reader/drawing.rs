#![allow(clippy::single_match)]

use std::io::Read;
use std::str::FromStr;

use xml::attribute::OwnedAttribute;
use xml::reader::{EventReader, XmlEvent};

use crate::{DrawingPositionType, RelativeFromHType, RelativeFromVType};

use super::*;

fn read_position_h<R: Read>(
    r: &mut EventReader<R>,
    attrs: &[OwnedAttribute],
) -> Result<(RelativeFromHType, i32), ReaderError> {
    let mut offset: i32 = 0;
    let mut relative_from_h = RelativeFromHType::default();

    loop {
        if let Some(h) = read(attrs, "relativeFrom") {
            if let Ok(h) = RelativeFromHType::from_str(&h) {
                relative_from_h = h;
            }
        }
        let e = r.next();
        match e {
            Ok(XmlEvent::Characters(c)) => {
                if let Ok(p) = i32::from_str(&c) {
                    offset = p;
                }
            }
            Ok(XmlEvent::EndElement { name, .. }) => {
                let e = WpXMLElement::from_str(&name.local_name).unwrap();
                if e == WpXMLElement::PositionH {
                    return Ok((relative_from_h, offset));
                }
            }
            Err(_) => return Err(ReaderError::XMLReadError),
            _ => {}
        }
    }
}

fn read_position_v<R: Read>(
    r: &mut EventReader<R>,
    attrs: &[OwnedAttribute],
) -> Result<(RelativeFromVType, i32), ReaderError> {
    let mut offset: i32 = 0;
    let mut relative_from_v = RelativeFromVType::default();
    loop {
        if let Some(v) = read(attrs, "relativeFrom") {
            if let Ok(v) = RelativeFromVType::from_str(&v) {
                relative_from_v = v;
            }
        }

        let e = r.next();
        match e {
            Ok(XmlEvent::Characters(c)) => {
                if let Ok(p) = i32::from_str(&c) {
                    offset = p;
                }
            }
            Ok(XmlEvent::EndElement { name, .. }) => {
                let e = WpXMLElement::from_str(&name.local_name).unwrap();
                if e == WpXMLElement::PositionV {
                    return Ok((relative_from_v, offset));
                }
            }
            Err(_) => return Err(ReaderError::XMLReadError),
            _ => {}
        }
    }
}

impl ElementReader for Drawing {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        _attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut drawing = Drawing::new();
        let mut drawing_position_type = DrawingPositionType::Inline;

        let mut simple_pos = false;
        let mut simple_pos_x = 0;
        let mut simple_pos_y = 0;
        let mut layout_in_cell = true;
        let mut relative_height = 0;
        let mut position_h = 0;
        let mut position_v = 0;
        let mut relative_from_h = RelativeFromHType::default();
        let mut relative_from_v = RelativeFromVType::default();
        let mut allow_overlap = true;
        let mut dist_t = 0;
        let mut dist_b = 0;
        let mut dist_l = 0;
        let mut dist_r = 0;

        loop {
            let e = r.next();
            match e {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    // wp:
                    if let Ok(wpe) = WpXMLElement::from_str(&name.local_name) {
                        match wpe {
                            WpXMLElement::Anchor => {
                                drawing_position_type = DrawingPositionType::Anchor;
                                if let Some(v) = read(&attributes, "simplePos") {
                                    if !is_false(&v) {
                                        simple_pos = true;
                                    }
                                }
                                if let Some(d) = read(&attributes, "distT") {
                                    if let Ok(d) = i32::from_str(&d) {
                                        dist_t = d;
                                    }
                                }
                                if let Some(d) = read(&attributes, "distB") {
                                    if let Ok(d) = i32::from_str(&d) {
                                        dist_b = d;
                                    }
                                }
                                if let Some(d) = read(&attributes, "distL") {
                                    if let Ok(d) = i32::from_str(&d) {
                                        dist_l = d;
                                    }
                                }
                                if let Some(d) = read(&attributes, "distR") {
                                    if let Ok(d) = i32::from_str(&d) {
                                        dist_r = d;
                                    }
                                }
                                if let Some(d) = read(&attributes, "layoutInCell") {
                                    if is_false(&d) {
                                        layout_in_cell = false;
                                    }
                                }
                                if let Some(d) = read(&attributes, "relativeHeight") {
                                    if let Ok(d) = u32::from_str(&d) {
                                        relative_height = d;
                                    }
                                }
                                if let Some(d) = read(&attributes, "allowOverlap") {
                                    if is_false(&d) {
                                        allow_overlap = false;
                                    }
                                }
                            }
                            WpXMLElement::Inline => {
                                drawing_position_type = DrawingPositionType::Inline;
                                if let Some(d) = read(&attributes, "distT") {
                                    if let Ok(d) = i32::from_str(&d) {
                                        dist_t = d;
                                    }
                                }
                                if let Some(d) = read(&attributes, "distB") {
                                    if let Ok(d) = i32::from_str(&d) {
                                        dist_b = d;
                                    }
                                }
                                if let Some(d) = read(&attributes, "distL") {
                                    if let Ok(d) = i32::from_str(&d) {
                                        dist_l = d;
                                    }
                                }
                                if let Some(d) = read(&attributes, "distR") {
                                    if let Ok(d) = i32::from_str(&d) {
                                        dist_r = d;
                                    }
                                }
                            }
                            WpXMLElement::SimplePos => {
                                if let Some(x) = read(&attributes, "x") {
                                    if let Ok(x) = i32::from_str(&x) {
                                        simple_pos_x = x;
                                    }
                                }
                                if let Some(y) = read(&attributes, "y") {
                                    if let Ok(y) = i32::from_str(&y) {
                                        simple_pos_y = y;
                                    }
                                }
                            }
                            WpXMLElement::PositionH => {
                                if let Ok(p) = read_position_h(r, &attributes) {
                                    relative_from_h = p.0;
                                    position_h = p.1;
                                }
                            }
                            WpXMLElement::PositionV => {
                                if let Ok(p) = read_position_v(r, &attributes) {
                                    relative_from_v = p.0;
                                    position_v = p.1;
                                }
                            }
                            _ => {}
                        }
                    }
                    // pic:
                    if let Ok(PicXMLElement::Pic) = PicXMLElement::from_str(&name.local_name) {
                        if let Ok(mut pic) = Pic::read(r, &attributes) {
                            pic.position_type = drawing_position_type;
                            pic.simple_pos = simple_pos;
                            pic.simple_pos_x = simple_pos_x;
                            pic.simple_pos_y = simple_pos_y;
                            pic.layout_in_cell = layout_in_cell;
                            pic.relative_height = relative_height;
                            pic.allow_overlap = allow_overlap;
                            pic.dist_r = dist_r;
                            pic.dist_t = dist_t;
                            pic.dist_b = dist_b;
                            pic.dist_l = dist_l;
                            pic.dist_r = dist_r;
                            pic.relative_from_h = relative_from_h;
                            pic.relative_from_v = relative_from_v;
                            pic.position_v = DrawingPosition::Offset(position_v);
                            pic.position_h = DrawingPosition::Offset(position_h);
                            drawing = drawing.pic(pic);
                        }
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    if e == XMLElement::Drawing {
                        return Ok(drawing);
                    }
                }
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
    }
}
