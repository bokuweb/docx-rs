use super::*;

use crate::documents::BuildXML;
use crate::xml_builder::*;

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    default_tab_stop: DefaultTabStop,
    zoom: Zoom,
    doc_id: Option<DocId>,
}

impl Settings {
    pub fn new() -> Settings {
        Default::default()
    }

    pub fn doc_id(mut self, id: impl Into<String>) -> Self {
        self.doc_id = Some(DocId::new(id.into()));
        self
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            default_tab_stop: DefaultTabStop::new(709),
            zoom: Zoom::new(100),
            doc_id: None,
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
            .open_compat()
            .space_for_ul()
            .balance_single_byte_double_byte_width()
            .do_not_leave_backslash_alone()
            .ul_trail_space()
            .do_not_expand_shift_return()
            .adjust_line_height_table()
            .use_fe_layout()
            .compat_setting(
                "compatibilityMode",
                "http://schemas.microsoft.com/office/word",
                "15",
            )
            .compat_setting(
                "overrideTableStyleFontSizeAndJustification",
                "http://schemas.microsoft.com/office/word",
                "1",
            )
            .compat_setting(
                "enableOpenTypeFeatures",
                "http://schemas.microsoft.com/office/word",
                "1",
            )
            .compat_setting(
                "doNotFlipMirrorIndents",
                "http://schemas.microsoft.com/office/word",
                "1",
            )
            .compat_setting(
                "differentiateMultirowTableHeaders",
                "http://schemas.microsoft.com/office/word",
                "1",
            )
            .compat_setting(
                "useWord2013TrackBottomHyphenation",
                "http://schemas.microsoft.com/office/word",
                "0",
            )
            .close()
            .add_optional_child(&self.doc_id)
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
<w:settings xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:w14="http://schemas.microsoft.com/office/word/2010/wordml" xmlns:w15="http://schemas.microsoft.com/office/word/2012/wordml"><w:defaultTabStop w:val="709" /><w:zoom w:percent="100" /><w:compat>
    <w:spaceForUL />
    <w:balanceSingleByteDoubleByteWidth />
    <w:doNotLeaveBackslashAlone />
    <w:ulTrailSpace />
    <w:doNotExpandShiftReturn />
    <w:adjustLineHeightInTable />
    <w:useFELayout />
    <w:compatSetting w:name="compatibilityMode" w:uri="http://schemas.microsoft.com/office/word" w:val="15" />
    <w:compatSetting w:name="overrideTableStyleFontSizeAndJustification" w:uri="http://schemas.microsoft.com/office/word" w:val="1" />
    <w:compatSetting w:name="enableOpenTypeFeatures" w:uri="http://schemas.microsoft.com/office/word" w:val="1" />
    <w:compatSetting w:name="doNotFlipMirrorIndents" w:uri="http://schemas.microsoft.com/office/word" w:val="1" />
    <w:compatSetting w:name="differentiateMultirowTableHeaders" w:uri="http://schemas.microsoft.com/office/word" w:val="1" />
    <w:compatSetting w:name="useWord2013TrackBottomHyphenation" w:uri="http://schemas.microsoft.com/office/word" w:val="0" />
  </w:compat>
</w:settings>"#
        );
    }
}
