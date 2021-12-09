use serde::Serialize;

use crate::documents::*;

// https://c-rex.net/projects/samples/ooxml/e1/Part4/OOXML_P4_DOCX_TOCTOC_topic_ID0ELZO1.html
#[derive(Serialize, Debug, Clone, PartialEq, Default)]
pub struct InstrToC {
    //  If no heading range is specified, all heading levels used in the document are listed.
    #[serde(skip_serializing_if = "Option::is_none")]
    heading_styles_range: Option<(usize, usize)>,
    // \l Includes TC fields that assign entries to one of the levels specified by text in this switch's field-argument as a range having the form startLevel-endLevel,
    //    where startLevel and endLevel are integers, and startLevel has a value equal-to or less-than endLevel.
    //    TC fields that assign entries to lower levels are skipped.
    #[serde(skip_serializing_if = "Option::is_none")]
    tc_field_level_range: Option<(usize, usize)>,
    // \n Without field-argument, omits page numbers from the table of contents.
    // .Page numbers are omitted from all levels unless a range of entry levels is specified by text in this switch's field-argument.
    // A range is specified as for \l.
    #[serde(skip_serializing_if = "Option::is_none")]
    omit_page_numbers_level_range: Option<(usize, usize)>,
    // \b includes entries only from the portion of the document marked by the bookmark named by text in this switch's field-argument.
    entry_bookmark_name: Option<String>,
    // \p text in this switch's field-argument specifies a sequence of characters that separate an entry and its page number.
    // .  The default is a tab with leader dots.
    separator_text: Option<String>,
    hyperlink: bool,
    preserve_tab: bool,
    preserve_new_line: bool,
    use_applied_paragraph_line_level: bool,
    // \z Hides tab leader and page numbers in Web layout view.
    hide_tab_and_page_numbers_in_webview: bool,
}

impl InstrToC {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn heading_styles_range(mut self, start: usize, end: usize) -> Self {
        self.heading_styles_range = Some((start, end));
        self
    }

    pub fn tc_field_level_range(mut self, start: usize, end: usize) -> Self {
        self.tc_field_level_range = Some((start, end));
        self
    }

    pub fn omit_page_numbers_level_range(mut self, start: usize, end: usize) -> Self {
        self.omit_page_numbers_level_range = Some((start, end));
        self
    }

    pub fn separator_text(mut self, t: impl Into<String>) -> Self {
        self.separator_text = Some(t.into());
        self
    }

    pub fn entry_bookmark_name(mut self, t: impl Into<String>) -> Self {
        self.entry_bookmark_name = Some(t.into());
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
}

impl BuildXML for InstrToC {
    fn build(&self) -> Vec<u8> {
        let mut instr = "TOC".to_string();

        if let Some(heading_styles_range) = self.heading_styles_range {
            instr = format!(
                "{} \\o &quot;{}-{}&quot;",
                instr, heading_styles_range.0, heading_styles_range.1
            );
        }

        if let Some(ref t) = self.separator_text {
            instr = format!("{} \\p &quot;{}&quot;", instr, t);
        }

        if self.hyperlink {
            instr = format!("{} \\h", instr);
        }

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
                    "\\o" => {
                        if let Some(r) = s.next() {
                            if let Some((s, e)) = parse_level_range(r) {
                                toc = toc.heading_styles_range(s, e);
                            }
                        }
                    }
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
                    "\\p" => {
                        if let Some(r) = s.next() {
                            let r = r.replace("&quot;", "").replace("\"", "");
                            toc = toc.separator_text(r);
                        }
                    }
                    "\\b" => {
                        if let Some(r) = s.next() {
                            let r = r.replace("&quot;", "").replace("\"", "");
                            toc = toc.entry_bookmark_name(r);
                        }
                    }
                    "\\u" => toc = toc.use_applied_paragraph_line_level(),
                    "\\h" => toc = toc.hyperlink(),
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
    fn read_toc_1() {
        let i = r#"TOC \o &quot;1-3&quot; \h"#;
        let i = InstrToC::from_str(i).unwrap();
        assert_eq!(i, InstrToC::new().heading_styles_range(1, 3).hyperlink());
    }

    #[test]
    fn read_toc_2() {
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
}
