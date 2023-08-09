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
                spacing = spacing.before(f64::from_str(&a.value)? as u32);
            }
            "after" => {
                spacing = spacing.after(f64::from_str(&a.value)? as u32);
            }
            "line" => {
                spacing = spacing.line(f64::from_str(&a.value)? as i32);
            }
            "lineRule" => {
                spacing = spacing.line_rule(LineSpacingType::from_str(&a.value)?);
            }
            "beforeLines" => {
                spacing = spacing.before_lines(f64::from_str(&a.value)? as u32);
            }
            "afterLines" => {
                spacing = spacing.after_lines(f64::from_str(&a.value)? as u32);
            }
            _ => {}
        }
    }
    Ok(spacing)
}
