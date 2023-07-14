#![allow(clippy::single_match)]

use std::io::Read;
use std::str::FromStr;

use xml::attribute::OwnedAttribute;
use xml::reader::{EventReader, XmlEvent};

use crate::types::*;
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
                if let Ok(p) = f64::from_str(&c) {
                    offset = p as i32;
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
                if let Ok(p) = f64::from_str(&c) {
                    offset = p as i32;
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

fn read_textbox_content<R: Read>(
    r: &mut EventReader<R>,
    _attrs: &[OwnedAttribute],
) -> Result<Vec<TextBoxContentChild>, ReaderError> {
    let mut children = vec![];
    loop {
        let e = r.next();
        match e {
            Ok(XmlEvent::StartElement {
                attributes, name, ..
            }) => {
                let e = XMLElement::from_str(&name.local_name).unwrap();
                match e {
                    XMLElement::Paragraph => {
                        let p = Paragraph::read(r, &attributes)?;
                        children.push(TextBoxContentChild::Paragraph(Box::new(p)));
                        continue;
                    }
                    XMLElement::Table => {
                        let t = Table::read(r, &attributes)?;
                        children.push(TextBoxContentChild::Table(Box::new(t)));
                        continue;
                    }
                    _ => {}
                }
            }
            Ok(XmlEvent::EndElement { name, .. }) => {
                let e = WpsXMLElement::from_str(&name.local_name).unwrap();
                if e == WpsXMLElement::Txbx {
                    return Ok(children);
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
                                    if let Ok(d) = f64::from_str(&d) {
                                        dist_t = d as i32;
                                    }
                                }
                                if let Some(d) = read(&attributes, "distB") {
                                    if let Ok(d) = f64::from_str(&d) {
                                        dist_b = d as i32;
                                    }
                                }
                                if let Some(d) = read(&attributes, "distL") {
                                    if let Ok(d) = f64::from_str(&d) {
                                        dist_l = d as i32;
                                    }
                                }
                                if let Some(d) = read(&attributes, "distR") {
                                    if let Ok(d) = f64::from_str(&d) {
                                        dist_r = d as i32;
                                    }
                                }
                                if let Some(d) = read(&attributes, "layoutInCell") {
                                    if is_false(&d) {
                                        layout_in_cell = false;
                                    }
                                }
                                if let Some(d) = read(&attributes, "relativeHeight") {
                                    if let Ok(d) = f64::from_str(&d) {
                                        relative_height = d as u32;
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
                                    if let Ok(d) = f64::from_str(&d) {
                                        dist_t = d as i32;
                                    }
                                }
                                if let Some(d) = read(&attributes, "distB") {
                                    if let Ok(d) = f64::from_str(&d) {
                                        dist_b = d as i32;
                                    }
                                }
                                if let Some(d) = read(&attributes, "distL") {
                                    if let Ok(d) = f64::from_str(&d) {
                                        dist_l = d as i32;
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
                                    if let Ok(x) = f64::from_str(&x) {
                                        simple_pos_x = x as i32;
                                    }
                                }
                                if let Some(y) = read(&attributes, "y") {
                                    if let Ok(y) = f64::from_str(&y) {
                                        simple_pos_y = y as i32;
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

                    // wps:
                    if let Ok(WpsXMLElement::Txbx) = WpsXMLElement::from_str(&name.local_name) {
                        if let Ok(children) = read_textbox_content(r, &attributes) {
                            let mut text_box = TextBox::new();
                            text_box.position_type = drawing_position_type;
                            text_box.simple_pos = simple_pos;
                            text_box.simple_pos_x = simple_pos_x;
                            text_box.simple_pos_y = simple_pos_y;
                            text_box.layout_in_cell = layout_in_cell;
                            text_box.relative_height = relative_height;
                            text_box.allow_overlap = allow_overlap;
                            text_box.dist_r = dist_r;
                            text_box.dist_t = dist_t;
                            text_box.dist_b = dist_b;
                            text_box.dist_l = dist_l;
                            text_box.dist_r = dist_r;
                            text_box.relative_from_h = relative_from_h;
                            text_box.relative_from_v = relative_from_v;
                            text_box.position_v = DrawingPosition::Offset(position_v);
                            text_box.position_h = DrawingPosition::Offset(position_h);
                            text_box.children = children;
                            drawing = drawing.text_box(text_box);
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
