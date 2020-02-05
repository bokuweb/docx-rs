use serde::{Deserialize, Serialize};

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Italic {}

impl Italic {
    pub fn new() -> Italic {
        Default::default()
    }
}

impl Default for Italic {
    fn default() -> Self {
        Self {}
    }
}

impl BuildXML for Italic {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.i().build()
    }
}
