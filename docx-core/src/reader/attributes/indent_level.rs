use std::str::FromStr;

use xml::attribute::OwnedAttribute;

use super::super::errors::*;

pub fn read_indent_level(attrs: &[OwnedAttribute]) -> Result<usize, ReaderError> {
    let mut l = 0;
    for a in attrs {
        let local_name = &a.name.local_name;
        if local_name == "ilvl" {
            l = usize::from_str(&a.value)?;
        }
    }
    Ok(l)
}
