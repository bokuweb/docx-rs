mod bool_value;
mod border;
mod id;
mod indent;
mod indent_level;
pub(crate) mod line_spacing;
mod name;
mod val;
mod width;

pub use bool_value::*;
pub use border::*;
pub use id::*;
pub use indent::*;
pub use indent_level::*;
pub use name::*;
pub use val::*;
pub use width::*;

use xml::attribute::OwnedAttribute;

pub fn read(attrs: &[OwnedAttribute], target: &str) -> Option<String> {
    for a in attrs {
        let local_name = &a.name.local_name;
        if local_name == target {
            return Some(a.value.to_owned());
        }
    }
    None
}
