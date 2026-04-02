use crate::reader::OwnedAttribute;

pub fn is_false(v: &str) -> bool {
    v == "0" || v.eq_ignore_ascii_case("false") || v.eq_ignore_ascii_case("off")
}

pub fn read_bool(attrs: &[OwnedAttribute]) -> bool {
    if let Some(v) = attrs
        .iter()
        .find(|attr| attr.name.local_name == "val")
        .or_else(|| attrs.first())
    {
        return !is_false(&v.value);
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reader::OwnedName;

    fn attr(local_name: &str, value: &str) -> OwnedAttribute {
        OwnedAttribute {
            name: OwnedName {
                local_name: local_name.to_string(),
                namespace: None,
                prefix: Some("w".to_string()),
            },
            value: value.to_string(),
        }
    }

    #[test]
    fn test_is_false_supports_off() {
        assert!(is_false("off"));
        assert!(is_false("OFF"));
    }

    #[test]
    fn test_read_bool_prefers_val_attribute() {
        let attrs = vec![attr("foo", "false"), attr("val", "on")];
        assert!(read_bool(&attrs));
    }
}
