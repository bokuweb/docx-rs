use serde::Serialize;

use crate::documents::*;

// https://c-rex.net/projects/samples/ooxml/e1/Part4/OOXML_P4_DOCX_TOCTOC_topic_ID0ELZO1.html
#[derive(Serialize, Debug, Clone, PartialEq, Default)]
pub struct InstrToC {
    //  If no heading range is specified, all heading levels used in the document are listed.
    #[serde(skip_serializing_if = "Option::is_none")]
    heading_styles_range: Option<(usize, usize)>,
    // \p text in this switch's field-argument specifies a sequence of characters that separate an entry and its page number.
    // .  The default is a tab with leader dots.
    separator_text: Option<String>,
    hyperlink: bool,
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

    pub fn separator_text(mut self, t: impl Into<String>) -> Self {
        self.separator_text = Some(t.into());
        self
    }

    pub fn hyperlink(mut self) -> Self {
        self.hyperlink = true;
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
                            let r = r.replace("&quot;", "").replace("\"", "");
                            let r: Vec<&str> = r.split('-').collect();
                            if let Some(s) = r.get(0) {
                                if let Ok(s) = usize::from_str(s) {
                                    if let Some(e) = r.get(1) {
                                        if let Ok(e) = usize::from_str(e) {
                                            toc = toc.heading_styles_range(s, e);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    "\\p" => {
                        if let Some(r) = s.next() {
                            let r = r.replace("&quot;", "").replace("\"", "");
                            toc = toc.separator_text(r);
                        }
                    }
                    "\\h" => toc = toc.hyperlink(),
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
    fn read_toc() {
        let i = r#"TOC \o &quot;1-3&quot; \h"#;
        let i = InstrToC::from_str(i).unwrap();
        assert_eq!(i, InstrToC::new().heading_styles_range(1, 3).hyperlink());
    }
}
