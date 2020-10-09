use xml::attribute::OwnedAttribute;

pub fn read_val(attrs: &[OwnedAttribute]) -> Option<String> {
    for a in attrs {
        let local_name = &a.name.local_name;
        if local_name == "val" {
            return Some(a.value.to_owned());
        }
    }
    None
}
