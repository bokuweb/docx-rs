use std::fmt;
use std::str::FromStr;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use serde::{Serialize, Deserialize};
use super::errors;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Copy, Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CharacterSpacingValues {
    DoNotCompress,
    CompressPunctuation,
    CompressPunctuationAndJapaneseKana,
    Unsupported,
}

impl fmt::Display for CharacterSpacingValues {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CharacterSpacingValues::DoNotCompress => write!(f, "doNotCompress"),
            CharacterSpacingValues::CompressPunctuation => write!(f, "compressPunctuation"),
            CharacterSpacingValues::CompressPunctuationAndJapaneseKana => write!(f, "compressPunctuationAndJapaneseKana"),
            _ => write!(f, "unsupported"),
        }
    }
}

impl FromStr for CharacterSpacingValues {
    type Err = errors::TypeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "doNotCompress" => Ok(CharacterSpacingValues::DoNotCompress),
            "compressPunctuation" => Ok(CharacterSpacingValues::CompressPunctuation),
            "compressPunctuationAndJapaneseKana" => Ok(CharacterSpacingValues::CompressPunctuationAndJapaneseKana),
            _ => Err(errors::TypeError::Unsupported(s.to_string()))
        }
    }
}
