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
        Option<i32>,
        Option<i32>,
    ),
    ReaderError,
>;

pub fn read_indent(attrs: &[OwnedAttribute]) -> ReadIndentResult {
    let mut start: Option<i32> = None;
    let mut start_chars: Option<i32> = None;
    let mut hanging_chars: Option<i32> = None;
    let mut first_line_chars: Option<i32> = None;
    let mut end: Option<i32> = None;
    let mut special: Option<SpecialIndentType> = None;
    for a in attrs {
        let local_name = &a.name.local_name;
        if local_name == "left" || local_name == "start" {
            let v = super::value_to_dax(&a.value)?;
            start = Some(v);
        } else if local_name == "leftChars" || local_name == "startChars" {
            start_chars = Some(i32::from_str(&a.value)?);
        } else if local_name == "end" || local_name == "right" {
            let v = super::value_to_dax(&a.value)?;
            end = Some(v);
        } else if local_name == "hanging" {
            let v = super::value_to_dax(&a.value)?;
            special = Some(SpecialIndentType::Hanging(v))
        } else if local_name == "firstLine" {
            let v = super::value_to_dax(&a.value)?;
            special = Some(SpecialIndentType::FirstLine(v))
        } else if local_name == "firstLineChars" {
            if let Ok(chars) = i32::from_str(&a.value) {
                first_line_chars = Some(chars);
            }
        } else if local_name == "hangingChars" {
            if let Ok(chars) = i32::from_str(&a.value) {
                hanging_chars = Some(chars);
            }
        }
    }
    Ok((
        start,
        end,
        special,
        start_chars,
        hanging_chars,
        first_line_chars,
    ))
}
