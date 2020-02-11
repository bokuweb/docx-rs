use super::{DefaultTabStop, Zoom};
use crate::documents::BuildXML;
use crate::xml_builder::*;

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    default_tab_stop: DefaultTabStop,
    zoom: Zoom,
}

impl Settings {
    pub fn new() -> Settings {
        Default::default()
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            default_tab_stop: DefaultTabStop::new(709),
            zoom: Zoom::new(100),
        }
    }
}

impl BuildXML for Settings {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.declaration(Some(true))
            .open_settings()
            .add_child(&self.default_tab_stop)
            .add_child(&self.zoom)
            .close()
            .build()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_settings() {
        let c = Settings::new();
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:settings xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:defaultTabStop w:val="709" /><w:zoom w:percent="100" /></w:settings>"#
        );
    }
}
