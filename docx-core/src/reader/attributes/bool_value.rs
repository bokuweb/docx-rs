use xml::attribute::OwnedAttribute;

pub fn read_bool(attrs: &[OwnedAttribute]) -> bool {
    if let Some(v) = attrs.get(0) {
        if &v.value == "0" || &v.value == "false" {
            return false;
        }
    }
    true
}
