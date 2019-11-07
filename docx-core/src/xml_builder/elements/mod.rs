#[macro_use]
mod macros;

pub mod based_on;
pub mod body;
pub mod color;
pub mod doc_defaults;
pub mod name;
pub mod next;
pub mod paragraph;
pub mod paragraph_property;
pub mod q_format;
pub mod run;
pub mod run_property;
pub mod run_property_default;
pub mod style;
pub mod sz;
pub mod text;

use super::{XMLBuilder, XmlEvent};
