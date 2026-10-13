#![allow(clippy::single_match)]

use std::io::Read;
use std::str::FromStr;

use super::*;

fn source_crop_offset(value: &str) -> Option<i32> {
    let value = value.trim();
    if let Some(percent) = value.strip_suffix('%') {
        let units = percent.trim().parse::<f64>().ok()? * 1000.0;
        if units.is_finite() && units >= i32::MIN as f64 && units <= i32::MAX as f64 {
            Some(units.round() as i32)
        } else {
            None
        }
    } else {
        value.parse().ok()
    }
}

impl ElementReader for Pic {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        _attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut pic = Pic::with_empty();
        let mut depth: usize = 0;
        // Callers usually consumed pic:pic; direct reader tests have not.
        let mut picture_depth: usize = 0;
        let mut blip_fill_depth: Option<usize> = None;
        loop {
            let e = r.next_event();
            match e {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    depth += 1;
                    let namespace = match name.local_name.as_str() {
                        "pic" | "blipFill" | "srcRect" => {
                            name.namespace.clone().or_else(|| r.namespace_uri(&name))
                        }
                        _ => None,
                    };
                    let is_picture_namespace = matches!(
                        namespace.as_deref(),
                        Some("http://schemas.openxmlformats.org/drawingml/2006/picture")
                            | Some("http://purl.oclc.org/ooxml/drawingml/picture")
                    );
                    if depth == 1 && is_picture_namespace && name.local_name == "pic" {
                        picture_depth = depth;
                    }
                    if depth == picture_depth + 1
                        && is_picture_namespace
                        && name.local_name == "blipFill"
                    {
                        blip_fill_depth = Some(depth);
                    }
                    if blip_fill_depth == Some(depth - 1)
                        && name.local_name == "srcRect"
                        && matches!(
                            namespace.as_deref(),
                            Some("http://schemas.openxmlformats.org/drawingml/2006/main")
                                | Some("http://purl.oclc.org/ooxml/drawingml/main")
                        )
                    {
                        let offset = |edge: &str| -> i32 {
                            attributes
                                .iter()
                                .find_map(|attribute| {
                                    (attribute.name.prefix.is_none()
                                        && attribute.name.local_name == edge)
                                        .then(|| source_crop_offset(&attribute.value))
                                        .flatten()
                                })
                                .unwrap_or(0)
                        };
                        pic = pic.crop(offset("l"), offset("t"), offset("r"), offset("b"));
                    }
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
                    if blip_fill_depth == Some(depth) {
                        blip_fill_depth = None;
                    }
                    depth = depth.saturating_sub(1);
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

    fn read_cropped_picture(contents: &str) -> Pic {
        let xml = format!(
            r#"<pic:pic xmlns:pic="http://schemas.openxmlformats.org/drawingml/2006/picture" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:foreign="urn:foreign">{contents}<pic:spPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="1714500" cy="952500"/></a:xfrm></pic:spPr></pic:pic>"#
        );
        Pic::read(&mut EventReader::new(xml.as_bytes()), &[]).unwrap()
    }

    #[test]
    fn picture_source_crops_keep_each_signed_offset() {
        for (attributes, offsets) in [
            (r#"l="25000" r="25000""#, [25000, 0, 25000, 0]),
            (r#"t="4604" r="1295" b="6879""#, [0, 4604, 1295, 6879]),
            (r#"l="-20000" t="15000""#, [-20000, 15000, 0, 0]),
            (r#"l="37.5%" b="-2.25%""#, [37500, 0, 0, -2250]),
            (
                r#"l=" 25000 " t="invalid" foreign:r="75000""#,
                [25000, 0, 0, 0],
            ),
        ] {
            let picture = read_cropped_picture(&format!(
                r#"<pic:blipFill><a:blip r:embed="rIdImage1"/><a:srcRect {attributes}/><a:stretch><a:fillRect/></a:stretch></pic:blipFill>"#
            ));
            assert_eq!(
                serde_json::to_value(&picture).unwrap()["crop"],
                serde_json::json!({"left": offsets[0], "top": offsets[1], "right": offsets[2], "bottom": offsets[3]})
            );
            assert_eq!(picture.id, "rIdImage1");
            assert_eq!(picture.size, (1714500, 952500));
        }
    }

    #[test]
    fn picture_source_crops_survive_read_write_read() {
        let original = read_cropped_picture(
            r#"<pic:blipFill><a:blip r:embed="rIdImage1"/><a:srcRect l="25777" t="4604" r="1295" b="6879"/></pic:blipFill>"#,
        );
        let built = String::from_utf8(original.build()).unwrap();
        assert!(built.contains(r#"<a:srcRect l="25777" t="4604" r="1295" b="6879""#));
        let built = built.replacen(
            "<pic:pic ",
            r#"<pic:pic xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" "#,
            1,
        );
        let reparsed = Pic::read(&mut EventReader::new(built.as_bytes()), &[]).unwrap();
        assert_eq!(
            serde_json::to_value(original).unwrap()["crop"],
            serde_json::to_value(reparsed).unwrap()["crop"]
        );
    }

    #[test]
    fn picture_source_crops_use_namespace_uris_and_blip_fill_scope() {
        let picture = read_cropped_picture(
            r#"<pic:blipFill><a:blip r:embed="rIdImage1"/><d:srcRect xmlns:d="http://schemas.openxmlformats.org/drawingml/2006/main" l="37500"/><foreign:srcRect l="75000"/></pic:blipFill><pic:extLst><pic:blipFill><a:srcRect l="90000"/></pic:blipFill></pic:extLst>"#,
        );
        assert_eq!(
            serde_json::to_value(picture).unwrap()["crop"],
            serde_json::json!({"left":37500,"top":0,"right":0,"bottom":0})
        );
    }

    #[test]
    fn picture_source_crops_leave_default_picture_json_unchanged() {
        for rectangle in [
            "",
            "<a:srcRect/>",
            r#"<a:srcRect l="0" t="0" r="0" b="0"/>"#,
        ] {
            let picture = read_cropped_picture(&format!(
                r#"<pic:blipFill><a:blip r:embed="rIdImage1"/>{rectangle}</pic:blipFill>"#
            ));
            assert!(serde_json::to_value(picture).unwrap().get("crop").is_none());
        }
    }

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
