use std::io::Read;
use std::str::FromStr;

use super::*;

impl ElementReader for FrameProperty {
    fn read<R: Read>(
        _r: &mut EventReader<R>,
        attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut fp = FrameProperty::new();
        for a in attrs {
            let local_name = &a.name.local_name;
            let e = XMLElement::from_str(local_name).unwrap();
            match e {
                XMLElement::Wrap => {
                    fp = fp.wrap(a.value.clone());
                }
                XMLElement::HeightRule => {
                    fp = fp.h_rule(a.value.clone());
                }
                XMLElement::HAnchor => {
                    fp = fp.h_anchor(a.value.clone());
                }
                XMLElement::VAnchor => {
                    fp = fp.v_anchor(a.value.clone());
                }
                XMLElement::HSpace => {
                    if let Ok(s) = f64::from_str(&a.value) {
                        fp = fp.h_space(s as i32)
                    }
                }
                XMLElement::VSpace => {
                    if let Ok(s) = f64::from_str(&a.value) {
                        fp = fp.v_space(s as i32)
                    }
                }
                XMLElement::XAlign => fp = fp.x_align(a.value.clone()),
                XMLElement::YAlign => fp = fp.y_align(a.value.clone()),
                XMLElement::W => {
                    if let Ok(s) = f64::from_str(&a.value) {
                        fp = fp.width(s as u32)
                    }
                }
                XMLElement::H => {
                    if let Ok(s) = f64::from_str(&a.value) {
                        fp = fp.height(s as u32)
                    }
                }
                XMLElement::X => {
                    if let Ok(s) = f64::from_str(&a.value) {
                        fp = fp.x(s as i32)
                    }
                }
                XMLElement::Y => {
                    if let Ok(s) = f64::from_str(&a.value) {
                        fp = fp.y(s as i32)
                    }
                }
                _ => {}
            }
        }
        Ok(fp)
    }
}
