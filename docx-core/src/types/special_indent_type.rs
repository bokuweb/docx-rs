#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;

// INFO: wasm-bindgen only allow c-style enum for now
//       Please convert typescript type to following type.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SpecialIndentType {
    FirstLine(i32),
    Hanging(i32),
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Serialize, Copy, Clone, Debug)]
pub enum SpecialIndentKind {
    FirstLine,
    Hanging,
}

impl Serialize for SpecialIndentType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            SpecialIndentType::FirstLine(s) => {
                let mut t = serializer.serialize_struct("FirstLine", 2)?;
                t.serialize_field("type", "firstLine")?;
                t.serialize_field("val", &s)?;
                t.end()
            }
            SpecialIndentType::Hanging(s) => {
                let mut t = serializer.serialize_struct("Hanging", 2)?;
                t.serialize_field("type", "hanging")?;
                t.serialize_field("val", &s)?;
                t.end()
            }
        }
    }
}
