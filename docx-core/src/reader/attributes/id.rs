use xml::attribute::OwnedAttribute;

pub fn read_id(attrs: &[OwnedAttribute]) -> Option<String> {
    for a in attrs {
        let local_name = &a.name.local_name;
        if local_name == "id" {
            return Some(a.value.to_owned());
        }
    }
    None
}
