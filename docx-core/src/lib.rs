mod documents;
#[allow(hidden_glob_reexports)] // should rename?
mod errors;
mod escape;
mod macros;
mod reader;
mod types;
pub mod xml;
mod xml_builder;
mod xml_json;
mod zipper;

pub use documents::*;
pub use errors::*;
pub use reader::*;
pub use types::*;
pub use xml_json::*;
