use xml::attribute::OwnedAttribute;

pub fn read_name(attrs: &[OwnedAttribute]) -> Option<String> {
    for a in attrs {
        let local_name = &a.name.local_name;
        if local_name == "name" {
            return Some(a.value.to_owned());
        }
    }
    None
}
