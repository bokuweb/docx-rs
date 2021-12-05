use serde::{Deserialize, Serialize};

//
// Please see https://c-rex.net/projects/samples/ooxml/e1/Part4/OOXML_P4_DOCX_ST_FldCharType_topic_ID0E6TU2.html#topic_ID0E6TU2
//
use std::fmt;
use std::str::FromStr;
use wasm_bindgen::prelude::*;

use super::errors;

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum FieldCharType {
    Begin,
    Separate,
    End,
    Unsupported,
}

impl fmt::Display for FieldCharType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FieldCharType::Begin => write!(f, "begin"),
            FieldCharType::Separate => write!(f, "separate"),
            FieldCharType::End => write!(f, "end"),
            FieldCharType::Unsupported => write!(f, "unsupported"),
        }
    }
}

impl FromStr for FieldCharType {
    type Err = errors::TypeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "begin" => Ok(FieldCharType::Begin),
            "separate" => Ok(FieldCharType::Separate),
            "end" => Ok(FieldCharType::End),
            _ => Ok(FieldCharType::Unsupported),
        }
    }
}
