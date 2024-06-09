use serde::Serialize;

use crate::documents::*;

#[derive(Serialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct InstrNUMPAGES {}

impl InstrNUMPAGES {
    pub fn new() -> Self {
        Self {}
    }
}

impl BuildXML for InstrNUMPAGES {
    fn build(&self) -> Vec<u8> {
        let instr = "NUMPAGES".to_owned();
        instr.into()
    }
}
