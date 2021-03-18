use std::io::Read;
use std::str::FromStr;

use crate::types::*;
use xml::attribute::OwnedAttribute;
use xml::reader::EventReader;

use super::*;

impl ElementReader for Shading {
    fn read<R: Read>(
        _r: &mut EventReader<R>,
        attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut shd = Shading::new();
        for a in attrs {
            let local_name = &a.name.local_name;
            if local_name == "val" {
                if let Ok(val) = ShdType::from_str(&a.value) {
                    shd = shd.shd_type(val);
                }
            } else if local_name == "color" {
                shd = shd.color(&a.value);
            } else if local_name == "fill" {
                shd = shd.fill(&a.value);
            }
        }
        Ok(shd)
    }
}
