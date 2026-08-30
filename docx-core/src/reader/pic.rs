#![allow(clippy::single_match)]

use std::io::Read;
use std::str::FromStr;

use super::*;

impl ElementReader for Pic {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        _attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut pic = Pic::with_empty();
        loop {
            let e = r.next_event();
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
                            AXMLElement::Xfrm => {
                                if let Some(rotation) = read(&attributes, "rot") {
                                    if let Ok(rotation) = i64::from_str(&rotation) {
                                        const UNITS_PER_DEGREE: i64 = 60_000;
                                        const FULL_TURN_UNITS: i64 = 360 * UNITS_PER_DEGREE;
                                        let normalized = rotation.rem_euclid(FULL_TURN_UNITS);
                                        let degrees = ((normalized + UNITS_PER_DEGREE / 2)
                                            / UNITS_PER_DEGREE)
                                            % 360;
                                        pic = pic.rotate(degrees as u16);
                                    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_picture_rotation_from_transform() {
        let xml = r#"<pic:pic xmlns:pic="http://schemas.openxmlformats.org/drawingml/2006/picture" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
            <pic:blipFill><a:blip r:embed="rId1"/></pic:blipFill>
            <pic:spPr>
                <a:xfrm rot="20933656">
                    <a:off x="0" y="0"/>
                    <a:ext cx="4366365" cy="4366365"/>
                </a:xfrm>
            </pic:spPr>
        </pic:pic>"#;
        let mut reader = EventReader::new(xml.as_bytes());

        let picture = Pic::read(&mut reader, &[]).expect("picture should parse");

        assert_eq!(picture.rot, 349);
    }
}
