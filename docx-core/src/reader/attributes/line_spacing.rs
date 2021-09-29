use crate::line_spacing_type::LineSpacingType;
use crate::ReaderError;
use std::str::FromStr;
use xml::attribute::OwnedAttribute;

pub type LineSpacingResult = Result<
    (
        Option<u32>,
        Option<u32>,
        Option<u32>,
        Option<LineSpacingType>,
    ),
    ReaderError,
>;

pub fn read_line_spacing(attributes: &[OwnedAttribute]) -> LineSpacingResult {
    let mut before: Option<u32> = None;
    let mut after: Option<u32> = None;
    let mut line: Option<u32> = None;
    let mut spacing_type: Option<LineSpacingType> = None;
    for a in attributes {
        let local_name = &a.name.local_name;
        if local_name == "before" {
            before = Some(u32::from_str(&a.value)?);
        } else if local_name == "after" {
            after = Some(u32::from_str(&a.value)?);
        } else if local_name == "line" {
            line = Some(u32::from_str(&a.value)?);
        } else if local_name == "lineRule" {
            spacing_type = Some(LineSpacingType::from_str(&a.value)?);
        }
    }
    Ok((before, after, line, spacing_type))
}
