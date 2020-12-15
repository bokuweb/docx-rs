use xml::attribute::OwnedAttribute;

pub fn is_false(v: &str) -> bool {
    v == "0" || v == "false"
}

pub fn read_bool(attrs: &[OwnedAttribute]) -> bool {
    if let Some(v) = attrs.get(0) {
        if is_false(&v.value) {
            return false;
        }
    }
    true
}
