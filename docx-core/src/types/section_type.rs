use serde::{Deserialize, Serialize};

//
// Please see CT_SectType
//
// <xsd:enumeration value="nextPage"/>
// <xsd:enumeration value="nextColumn"/>
// <xsd:enumeration value="continuous"/>
// <xsd:enumeration value="evenPage"/>
// <xsd:enumeration value="oddPage"/>

use std::fmt;
use std::str::FromStr;
use wasm_bindgen::prelude::*;

use super::errors;

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum SectionType {
    NextPage,
    NextColumn,
    Continuous,
    EvenPage,
    OddPage,
}

impl fmt::Display for SectionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SectionType::NextPage => write!(f, "nextPage"),
            SectionType::NextColumn => write!(f, "nextColumn"),
            SectionType::Continuous => write!(f, "continuous"),
            SectionType::EvenPage => write!(f, "evenPage"),
            SectionType::OddPage => write!(f, "oddPage"),
        }
    }
}

impl FromStr for SectionType {
    type Err = errors::TypeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "nextPage" => Ok(SectionType::NextPage),
            "nextColumn" => Ok(SectionType::NextColumn),
            "continuous" => Ok(SectionType::Continuous),
            "evenPage" => Ok(SectionType::EvenPage),
            "oddPage" => Ok(SectionType::OddPage),
            _ => Ok(SectionType::Continuous),
        }
    }
}
