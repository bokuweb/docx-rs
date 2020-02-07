use std::str::FromStr;

use xml::attribute::OwnedAttribute;

use crate::types::*;

use super::super::errors::*;

pub fn read_width(attrs: &[OwnedAttribute]) -> Result<(usize, WidthType), ReaderError> {
    let mut w = 0;
    let mut width_type = WidthType::Auto;
    for a in attrs {
        let local_name = &a.name.local_name;
        if local_name == "type" {
            width_type = WidthType::from_str(&a.value)?;
        } else if local_name == "w" {
            w = usize::from_str(&a.value)?;
        }
    }
    Ok((w, width_type))
}
