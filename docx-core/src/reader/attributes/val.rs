use std::str::FromStr;

use xml::attribute::OwnedAttribute;

use super::super::errors::*;

pub fn read_val(attrs: &[OwnedAttribute]) -> Option<String> {
    for a in attrs {
        let local_name = &a.name.local_name;
        if local_name == "val" {
            return Some(a.value.to_owned());
        }
    }
    None
}

pub fn value_to_dax(v: &str) -> Result<i32, ReaderError> {
    if v.ends_with("pt") {
        let v = f64::from_str(&v.replace("pt", ""))? as i32;
        Ok(v * 20)
    } else {
        Ok(f64::from_str(v)? as i32)
    }
}
