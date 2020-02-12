use std::str::FromStr;

use xml::attribute::OwnedAttribute;

use crate::types::*;

use super::super::errors::*;

pub fn read_indent(
    attrs: &[OwnedAttribute],
) -> Result<(usize, Option<usize>, Option<SpecialIndentType>), ReaderError> {
    let mut start = 0;
    let mut end: Option<usize> = None;
    let mut special: Option<SpecialIndentType> = None;

    for a in attrs {
        let local_name = &a.name.local_name;
        if local_name == "left" || local_name == "start" {
            start = usize::from_str(&a.value)?;
        } else if local_name == "end" || local_name == "right" {
            end = Some(usize::from_str(&a.value)?);
        } else if local_name == "hanging" {
            special = Some(SpecialIndentType::Hanging(usize::from_str(&a.value)?))
        } else if local_name == "firstLine" {
            special = Some(SpecialIndentType::FirstLine(usize::from_str(&a.value)?))
        }
    }

    Ok((start, end, special))
}
