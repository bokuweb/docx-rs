use std::str::FromStr;

use xml::attribute::OwnedAttribute;

use crate::types::*;

use super::super::errors::*;

pub type ReadIndentResult = Result<
    (
        Option<i32>,
        Option<i32>,
        Option<SpecialIndentType>,
        Option<i32>,
    ),
    ReaderError,
>;

pub fn read_indent(attrs: &[OwnedAttribute]) -> ReadIndentResult {
    let mut start: Option<i32> = None;
    let mut start_chars: Option<i32> = None;
    let mut end: Option<i32> = None;
    let mut special: Option<SpecialIndentType> = None;

    for a in attrs {
        let local_name = &a.name.local_name;
        if local_name == "left" || local_name == "start" {
            start = Some(i32::from_str(&a.value)?);
        } else if local_name == "leftChars" || local_name == "startChars" {
            start_chars = Some(i32::from_str(&a.value)?);
        } else if local_name == "end" || local_name == "right" {
            end = Some(i32::from_str(&a.value)?);
        } else if local_name == "hanging" {
            special = Some(SpecialIndentType::Hanging(i32::from_str(&a.value)?))
        } else if local_name == "firstLine" {
            special = Some(SpecialIndentType::FirstLine(i32::from_str(&a.value)?))
        }
    }

    Ok((start, end, special, start_chars))
}
