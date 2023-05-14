use serde::Serialize;

use crate::documents::*;

#[derive(Serialize, Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "wasm", derive(ts_rs::TS))]
#[cfg_attr(feature = "wasm", ts(export))]
pub struct StyleWithLevel(pub (String, usize));

impl StyleWithLevel {
    pub fn new(s: impl Into<String>, l: usize) -> Self {
        Self((s.into(), l))
    }
}
// https://c-rex.net/projects/samples/ooxml/e1/Part4/OOXML_P4_DOCX_TOCTOC_topic_ID0ELZO1.html
#[derive(Serialize, Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "wasm", derive(ts_rs::TS))]
#[cfg_attr(feature = "wasm", ts(export))]
#[serde(rename_all = "camelCase")]
pub struct InstrToC {
    // \o If no heading range is specified, all heading levels used in the document are listed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading_styles_range: Option<(usize, usize)>,
    // \l Includes TC fields that assign entries to one of the levels specified by text in this switch's field-argument as a range having the form startLevel-endLevel,
    //    where startLevel and endLevel are integers, and startLevel has a value equal-to or less-than endLevel.
    //    TC fields that assign entries to lower levels are skipped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tc_field_level_range: Option<(usize, usize)>,
    // \n Without field-argument, omits page numbers from the table of contents.
    // .Page numbers are omitted from all levels unless a range of entry levels is specified by text in this switch's field-argument.
    // A range is specified as for \l.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub omit_page_numbers_level_range: Option<(usize, usize)>,
    // \b includes entries only from the portion of the document marked by the bookmark named by text in this switch's field-argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_bookmark_name: Option<String>,
    // \t Uses paragraphs formatted with styles other than the built-in heading styles.
    // .  text in this switch's field-argument specifies those styles as a set of comma-separated doublets,
    //    with each doublet being a comma-separated set of style name and table of content level. \t can be combined with \o.
    pub styles_with_levels: Vec<StyleWithLevel>,
    //  struct S texWin Lis switch's field-argument specifies a sequence of character
    // .  The default is a tab with leader dots.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_and_page_number_separator: Option<String>,
    // \d
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_and_page_numbers_separator: Option<String>,
    // \a
    pub caption_label: Option<String>,
    // \c
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_label_including_numbers: Option<String>,
    // \s
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seq_field_identifier_for_prefix: Option<String>,
    // \f
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tc_field_identifier: Option<String>,
    // \h
    pub hyperlink: bool,
    // \w
    pub preserve_tab: bool,
    // \x
    pub preserve_new_line: bool,
    // \u
    pub use_applied_paragraph_line_level: bool,
    // \z Hides tab leader and page numbers in Web layout view.
    pub hide_tab_and_page_numbers_in_webview: bool,
}

impl InstrToC {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_instr_text(s: &str) -> Self {
        Self::from_str(s).expect("should convert to InstrToC")
    }

    pub fn heading_styles_range(mut self, start: usize, end: usize) -> Self {
        self.heading_styles_range = Some((start, end));
        self
    }

    pub fn tc_field_level_range(mut self, start: usize, end: usize) -> Self {
        self.tc_field_level_range = Some((start, end));
        self
    }

    pub fn tc_field_identifier(mut self, t: impl Into<String>) -> Self {
        self.tc_field_identifier = Some(t.into());
        self
    }

    pub fn omit_page_numbers_level_range(mut self, start: usize, end: usize) -> Self {
        self.omit_page_numbers_level_range = Some((start, end));
        self
    }

    pub fn entry_and_page_number_separator(mut self, t: impl Into<String>) -> Self {
        self.entry_and_page_number_separator = Some(t.into());
        self
    }

    pub fn entry_bookmark_name(mut self, t: impl Into<String>) -> Self {
        self.entry_bookmark_name = Some(t.into());
        self
    }

    pub fn caption_label(mut self, t: impl Into<String>) -> Self {
        self.caption_label = Some(t.into());
        self
    }

    pub fn caption_label_including_numbers(mut self, t: impl Into<String>) -> Self {
        self.caption_label_including_numbers = Some(t.into());
        self
    }

    pub fn sequence_and_page_numbers_separator(mut self, t: impl Into<String>) -> Self {
        self.sequence_and_page_numbers_separator = Some(t.into());
        self
    }

    pub fn seq_field_identifier_for_prefix(mut self, t: impl Into<String>) -> Self {
        self.seq_field_identifier_for_prefix = Some(t.into());
        self
    }

    pub fn hyperlink(mut self) -> Self {
        self.hyperlink = true;
        self
    }

    pub fn preserve_tab(mut self) -> Self {
        self.preserve_tab = true;
        self
    }

    pub fn preserve_new_line(mut self) -> Self {
        self.preserve_new_line = true;
        self
    }

    pub fn use_applied_paragraph_line_level(mut self) -> Self {
        self.use_applied_paragraph_line_level = true;
        self
    }

    pub fn hide_tab_and_page_numbers_in_webview(mut self) -> Self {
        self.hide_tab_and_page_numbers_in_webview = true;
        self
    }

    pub fn add_style_with_level(mut self, s: StyleWithLevel) -> Self {
        self.styles_with_levels.push(s);
        self
    }
}

impl BuildXML for InstrToC {
    fn build(&self) -> Vec<u8> {
        let mut instr = "TOC".to_string();

        // \a
        if let Some(ref t) = self.caption_label {
            instr = format!("{} \\a &quot;{}&quot;", instr, t);
        }

        // \b
        if let Some(ref t) = self.entry_bookmark_name {
            instr = format!("{} \\b &quot;{}&quot;", instr, t);
        }

        // \c
        if let Some(ref t) = self.caption_label_including_numbers {
            instr = format!("{} \\c &quot;{}&quot;", instr, t);
        }

        // \d
        if let Some(ref t) = self.sequence_and_page_numbers_separator {
            instr = format!("{} \\d &quot;{}&quot;", instr, t);
        }

        // \f
        if let Some(ref t) = self.tc_field_identifier {
            instr = format!("{} \\f &quot;{}&quot;", instr, t);
        }

        // \l
        if let Some(range) = self.tc_field_level_range {
            instr = format!("{} \\l &quot;{}-{}&quot;", instr, range.0, range.1);
        }

        // \n
        if let Some(range) = self.omit_page_numbers_level_range {
            instr = format!("{} \\n &quot;{}-{}&quot;", instr, range.0, range.1);
        }

        // \o
        if let Some(range) = self.heading_styles_range {
            instr = format!("{} \\o &quot;{}-{}&quot;", instr, range.0, range.1);
        }

        // \p
        if let Some(ref t) = self.entry_and_page_number_separator {
            instr = format!("{} \\p &quot;{}&quot;", instr, t);
        }

        // \s
        if let Some(ref t) = self.seq_field_identifier_for_prefix {
            instr = format!("{} \\s &quot;{}&quot;", instr, t);
        }

        // \t
        if !self.styles_with_levels.is_empty() {
            let s = self
                .styles_with_levels
                .iter()
                .map(|s| format!("{},{}", (s.0).0, (s.0).1))
                .collect::<Vec<String>>()
                .join(",");
            instr = format!("{} \\t &quot;{}&quot;", instr, s);
        }

        // \h
        if self.hyperlink {
            instr = format!("{} \\h", instr);
        }

        // \u
        if self.use_applied_paragraph_line_level {
            instr = format!("{} \\u", instr);
        }

        // \w
        if self.preserve_tab {
            instr = format!("{} \\w", instr);
        }

        // \x
        if self.preserve_new_line {
            instr = format!("{} \\x", instr);
        }

        // \z
        if self.hide_tab_and_page_numbers_in_webview {
            instr = format!("{} \\z", instr);
        }

        instr.into()
    }
}

fn parse_level_range(i: &str) -> Option<(usize, usize)> {
    let r = i.replace("&quot;", "").replace("\"", "");
    let r: Vec<&str> = r.split('-').collect();
    if let Some(s) = r.get(0) {
        if let Ok(s) = usize::from_str(s) {
            if let Some(e) = r.get(1) {
                if let Ok(e) = usize::from_str(e) {
                    return Some((s, e));
                }
            }
        }
    }
    None
}

impl std::str::FromStr for InstrToC {
    type Err = ();

    fn from_str(instr: &str) -> Result<Self, Self::Err> {
        let mut s = instr.split(' ');
        let mut toc = InstrToC::new();
        loop {
            if let Some(i) = s.next() {
                match i {
                    "\\a" => {
                        if let Some(r) = s.next() {
                            let r = r.replace("&quot;", "").replace("\"", "");
                            toc = toc.caption_label(r);
                        }
                    }
                    "\\b" => {
                        if let Some(r) = s.next() {
                            let r = r.replace("&quot;", "").replace("\"", "");
                            toc = toc.entry_bookmark_name(r);
                        }
                    }
                    "\\c" => {
                        if let Some(r) = s.next() {
                            let r = r.replace("&quot;", "").replace("\"", "");
                            toc = toc.caption_label_including_numbers(r);
                        }
                    }
                    "\\d" => {
                        if let Some(r) = s.next() {
                            let r = r.replace("&quot;", "").replace("\"", "");
                            toc = toc.sequence_and_page_numbers_separator(r);
                        }
                    }
                    "\\f" => {
                        if let Some(r) = s.next() {
                            let r = r.replace("&quot;", "").replace("\"", "");
                            toc = toc.tc_field_identifier(r);
                        }
                    }
                    "\\h" => toc = toc.hyperlink(),
                    "\\l" => {
                        if let Some(r) = s.next() {
                            if let Some((s, e)) = parse_level_range(r) {
                                toc = toc.tc_field_level_range(s, e);
                            }
                        }
                    }
                    "\\n" => {
                        if let Some(r) = s.next() {
                            if let Some((s, e)) = parse_level_range(r) {
                                toc = toc.omit_page_numbers_level_range(s, e);
                            }
                        }
                    }
                    "\\o" => {
                        if let Some(r) = s.next() {
                            if let Some((s, e)) = parse_level_range(r) {
                                toc = toc.heading_styles_range(s, e);
                            }
                        }
                    }
                    "\\p" => {
                        if let Some(r) = s.next() {
                            let r = r.replace("&quot;", "").replace("\"", "");
                            toc = toc.entry_and_page_number_separator(r);
                        }
                    }
                    "\\s" => {
                        if let Some(r) = s.next() {
                            let r = r.replace("&quot;", "").replace("\"", "");
                            toc = toc.seq_field_identifier_for_prefix(r);
                        }
                    }
                    "\\t" => {
                        if let Some(r) = s.next() {
                            let r = r.replace("&quot;", "").replace("\"", "");
                            let mut r = r.split(',');
                            loop {
                                if let Some(style) = r.next() {
                                    if let Some(level) = r.next() {
                                        if let Ok(level) = usize::from_str(level) {
                                            toc = toc.add_style_with_level(StyleWithLevel((
                                                style.to_string(),
                                                level,
                                            )));
                                            continue;
                                        }
                                    }
                                }
                                break;
                            }
                        }
                    }
                    "\\u" => toc = toc.use_applied_paragraph_line_level(),
                    "\\w" => toc = toc.preserve_tab(),
                    "\\x" => toc = toc.preserve_new_line(),
                    "\\z" => toc = toc.hide_tab_and_page_numbers_in_webview(),
                    _ => {}
                }
            } else {
                return Ok(toc);
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_toc() {
        let b = InstrToC::new().heading_styles_range(1, 3).build();
        assert_eq!(str::from_utf8(&b).unwrap(), r#"TOC \o &quot;1-3&quot;"#);
    }

    #[test]
    fn test_toc_with_styles() {
        let b = InstrToC::new()
            .heading_styles_range(1, 3)
            .add_style_with_level(StyleWithLevel::new("style1", 2))
            .add_style_with_level(StyleWithLevel::new("style2", 3))
            .build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"TOC \o &quot;1-3&quot; \t &quot;style1,2,style2,3&quot;"#
        );
    }

    #[test]
    fn read_toc_with_o_and_h() {
        let i = r#"TOC \o &quot;1-3&quot; \h"#;
        let i = InstrToC::from_str(i).unwrap();
        assert_eq!(i, InstrToC::new().heading_styles_range(1, 3).hyperlink());
    }

    #[test]
    fn read_toc_with_l_and_n() {
        let i = r#"TOC \o &quot;1-3&quot; \l &quot;4-5&quot; \n &quot;1-4&quot; \h"#;
        let i = InstrToC::from_str(i).unwrap();
        assert_eq!(
            i,
            InstrToC::new()
                .heading_styles_range(1, 3)
                .hyperlink()
                .omit_page_numbers_level_range(1, 4)
                .tc_field_level_range(4, 5)
        );
    }

    #[test]
    fn read_toc_with_a_and_b_and_t() {
        let i = r#"TOC \a &quot;hoge&quot; \b &quot;test&quot; \o &quot;1-3&quot; \t &quot;MySpectacularStyle,1,MySpectacularStyle2,4&quot;"#;
        let i = InstrToC::from_str(i).unwrap();
        assert_eq!(
            i,
            InstrToC::new()
                .caption_label("hoge")
                .entry_bookmark_name("test")
                .heading_styles_range(1, 3)
                .add_style_with_level(StyleWithLevel::new("MySpectacularStyle", 1))
                .add_style_with_level(StyleWithLevel::new("MySpectacularStyle2", 4))
        );
    }

    #[test]
    fn with_instr_text() {
        let s = r#"TOC \o "1-3" \h \z \u"#;
        let i = InstrToC::with_instr_text(s);
        assert_eq!(
            i,
            InstrToC::new()
                .heading_styles_range(1, 3)
                .use_applied_paragraph_line_level()
                .hide_tab_and_page_numbers_in_webview()
                .hyperlink()
        );
    }
}
