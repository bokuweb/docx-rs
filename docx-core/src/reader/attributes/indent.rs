use std::str::FromStr;

use xml::attribute::OwnedAttribute;

use crate::types::*;

use super::super::errors::*;

pub fn read_indent(
    attrs: &[OwnedAttribute],
) -> Result<(i32, Option<i32>, Option<SpecialIndentType>), ReaderError> {
    let mut start = 0;
    let mut end: Option<i32> = None;
    let mut special: Option<SpecialIndentType> = None;

    for a in attrs {
        let local_name = &a.name.local_name;
        if local_name == "left" || local_name == "start" {
            start = i32::from_str(&a.value)?;
        } else if local_name == "end" || local_name == "right" {
            end = Some(i32::from_str(&a.value)?);
        } else if local_name == "hanging" {
            special = Some(SpecialIndentType::Hanging(i32::from_str(&a.value)?))
        } else if local_name == "firstLine" {
            special = Some(SpecialIndentType::FirstLine(i32::from_str(&a.value)?))
        }
    }

    Ok((start, end, special))
}
