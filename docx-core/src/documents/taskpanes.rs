use serde::{Deserialize, Serialize};

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub struct Taskpanes {}

impl Taskpanes {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Default for Taskpanes {
    fn default() -> Self {
        Self {}
    }
}

impl BuildXML for Taskpanes {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        let b = b
            .declaration(Some(true))
            .open_taskpanes("http://schemas.microsoft.com/office/webextensions/taskpanes/2010/11")
            .open_taskpane("", "1", "350", "1")
            .webextensionref(
                "http://schemas.openxmlformats.org/officeDocument/2006/relationships",
                "rId1",
            )
            .close()
            .close();
        b.build()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_build() {
        let c = Taskpanes::new();
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<wetp:taskpanes xmlns:wetp="http://schemas.microsoft.com/office/webextensions/taskpanes/2010/11">
  <wetp:taskpane dockstate="" visibility="1" width="350" row="1">
    <wetp:webextensionref xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" r:id="rId1" />
  </wetp:taskpane>
</wetp:taskpanes>"#
        );
    }
}
