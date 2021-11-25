use crate::line_spacing_type::LineSpacingType;
use crate::LineSpacing;
use crate::ReaderError;
use std::str::FromStr;
use xml::attribute::OwnedAttribute;

pub fn read_line_spacing(attributes: &[OwnedAttribute]) -> Result<LineSpacing, ReaderError> {
    let mut spacing = LineSpacing::new();
    for a in attributes {
        let local_name = &a.name.local_name;
        match local_name.as_str() {
            "before" => {
                spacing = spacing.before(u32::from_str(&a.value)?);
            }
            "after" => {
                spacing = spacing.after(u32::from_str(&a.value)?);
            }
            "line" => {
                spacing = spacing.line(u32::from_str(&a.value)?);
            }
            "lineRule" => {
                spacing = spacing.line_rule(LineSpacingType::from_str(&a.value)?);
            }
            "beforeLines" => {
                spacing = spacing.before_lines(u32::from_str(&a.value)?);
            }
            "afterLines" => {
                spacing = spacing.after_lines(u32::from_str(&a.value)?);
            }
            _ => {}
        }
    }
    Ok(spacing)
}
