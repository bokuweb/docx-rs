use super::*;

use crate::documents::BuildXML;
use crate::types::CharacterSpacingValues;
use crate::xml_builder::*;

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    // Sequential child elements
    // 3. w:zoom
    zoom: Zoom,
    // 39. w:defaultTabStop
    default_tab_stop: DefaultTabStop,
    // 48. w:evenAndOddHeaders
    even_and_odd_headers: bool,
    // 61. w:characterSpacingControl
    #[serde(skip_serializing_if = "Option::is_none")]
    character_spacing_control: Option<CharacterSpacingValues>,
    // 81. child element of w:compat
    adjust_line_height_in_table: bool,
    // 82. w:docVars
    doc_vars: Vec<DocVar>,

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

    pub fn default_tab_stop(mut self, tab_stop: usize) -> Self {
        self.default_tab_stop = DefaultTabStop::new(tab_stop);
        self
    }

    pub fn add_doc_var(mut self, name: impl Into<String>, val: impl Into<String>) -> Self {
        self.doc_vars.push(DocVar::new(name, val));
        self
    }

    pub fn even_and_odd_headers(mut self) -> Self {
        self.even_and_odd_headers = true;
        self
    }

    pub fn adjust_line_height_in_table(mut self) -> Self {
        self.adjust_line_height_in_table = true;
        self
    }

    pub fn character_spacing_control(mut self, val: CharacterSpacingValues) -> Self {
        self.character_spacing_control = Some(val);
        self
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            default_tab_stop: DefaultTabStop::new(840),
            zoom: Zoom::new(100),
            doc_id: None,
            doc_vars: vec![],
            even_and_odd_headers: false,
            adjust_line_height_in_table: false,
            character_spacing_control: None,
        }
    }
}

impl BuildXML for Settings {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        let mut b = b
            .declaration(Some(true))
            .open_settings()
            .add_child(&self.zoom)
            .add_child(&self.default_tab_stop);

        if self.even_and_odd_headers {
            b = b.even_and_odd_headers();
        }

        if let Some(v) = self.character_spacing_control {
            b = b.character_spacing_control(&v.to_string());
        }

        if self.adjust_line_height_in_table {
            b = b.adjust_line_height_table();
        }

        b = b
            .open_compat()
            .space_for_ul()
            .balance_single_byte_double_byte_width()
            .do_not_leave_backslash_alone()
            .ul_trail_space()
            .do_not_expand_shift_return();

        b = b
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
            .add_optional_child(&self.doc_id);

        if !self.doc_vars.is_empty() {
            b = b.open_doc_vars();
            for v in self.doc_vars.iter() {
                b = b.add_child(v);
            }
            b = b.close();
        }

        b.close().build()
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
<w:settings xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:w14="http://schemas.microsoft.com/office/word/2010/wordml" xmlns:w15="http://schemas.microsoft.com/office/word/2012/wordml"><w:zoom w:percent="100" /><w:defaultTabStop w:val="840" /><w:compat>
    <w:spaceForUL />
    <w:balanceSingleByteDoubleByteWidth />
    <w:doNotLeaveBackslashAlone />
    <w:ulTrailSpace />
    <w:doNotExpandShiftReturn />
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
