#[macro_use]
mod macros;

mod based_on;
mod color;
mod doc_defaults;
mod name;
mod next;
mod paragraph_property;
mod q_format;
mod run_property;
mod run_property_default;
mod style;
mod sz;

pub use based_on::*;
pub use color::*;
pub use doc_defaults::*;
pub use name::*;
pub use next::*;
pub use paragraph_property::*;
pub use q_format::*;
pub use run_property::*;
pub use run_property_default::*;
pub use style::*;
pub use sz::*;

use super::XMLBuilder;
use super::XmlEvent;
