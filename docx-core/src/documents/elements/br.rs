use serde::{Deserialize, Serialize};

use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Break {
    break_type: BreakType,
}

impl Break {
    pub fn new(t: BreakType) -> Break {
        Break { break_type: t }
    }
}

impl BuildXML for Break {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.br(&self.break_type.to_string()).build()
    }
}
