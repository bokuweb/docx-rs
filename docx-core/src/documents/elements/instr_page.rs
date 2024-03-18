use serde::Serialize;

use crate::documents::*;

#[derive(Serialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct InstrPAGE {}

impl InstrPAGE {
    pub fn new() -> Self {
        Self {}
    }
}

impl BuildXML for InstrPAGE {
    fn build(&self) -> Vec<u8> {
        let instr = "PAGE".to_owned();
        instr.into()
    }
}
