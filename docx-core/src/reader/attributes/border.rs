use std::str::FromStr;

use xml::attribute::OwnedAttribute;

use crate::types::*;

use super::super::errors::*;

pub struct BorderAttrs {
    pub border_type: BorderType,
    pub color: String,
    pub size: Option<u32>,
    pub space: Option<u32>,
}

pub fn read_border(attrs: &[OwnedAttribute]) -> Result<BorderAttrs, ReaderError> {
    let mut border_type = BorderType::Single;
    let mut color = "000000".to_owned();
    let mut size: Option<u32> = Some(4);
    let mut space: Option<u32> = Some(0);
    for a in attrs {
        let local_name = &a.name.local_name;
        if local_name == "color" {
            color = a.value.to_owned();
        } else if local_name == "sz" {
            size = Some(u32::from_str(&a.value)?);
        } else if local_name == "space" {
            space = Some(u32::from_str(&a.value)?);
        } else if local_name == "val" {
            border_type = BorderType::from_str(&a.value)?;
        }
    }
    Ok(BorderAttrs {
        border_type,
        color,
        size,
        space,
    })
}
