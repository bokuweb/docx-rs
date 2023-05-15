use serde::Serialize;

// https://c-rex.net/projects/samples/ooxml/e1/Part4/OOXML_P4_DOCX_HYPERLINKHYPERLINK_topic_ID0EFYG1.html
#[derive(Serialize, Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "wasm", derive(ts_rs::TS))]
#[cfg_attr(feature = "wasm", ts(export))]
#[serde(rename_all = "camelCase")]
pub struct InstrHyperlink {
    pub target: String,
    // \l
    pub anchor: bool,
}

impl InstrHyperlink {
    pub fn new(target: impl Into<String>) -> Self {
        Self {
            target: target.into(),
            ..Default::default()
        }
    }
}

// impl BuildXML for instrHyperlink {
//     fn build(&self) -> Vec<u8> {
// TODO:
//     }
// }

impl std::str::FromStr for InstrHyperlink {
    type Err = ();

    fn from_str(instr: &str) -> Result<Self, Self::Err> {
        let mut s = instr.split(' ');
        let mut target = "".to_string();
        let mut anchor = false;
        loop {
            if let Some(i) = s.next() {
                match i {
                    "\\l" => {
                        anchor = true;
                    }
                    "\\m" => {
                        // TODO:
                    }
                    "\\n" => {
                        // TODO:
                    }
                    "\\o" => {
                        // TODO: Support later
                        let _ = s.next();
                    }
                    "\\t" => {
                        // TODO: Support later
                        let _ = s.next();
                    }
                    _ => {
                        target = i.replace("&quot;", "").replace("\"", "").to_string();
                    }
                }
            } else {
                // FIXME: For now, return error if target is not found
                if target.is_empty() {
                    return Err(());
                }
                return Ok(Self { target, anchor });
            }
        }
    }
}
