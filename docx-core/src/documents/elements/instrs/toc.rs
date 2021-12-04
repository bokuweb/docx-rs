use serde::Serialize;

use crate::documents::*;

// https://c-rex.net/projects/samples/ooxml/e1/Part4/OOXML_P4_DOCX_TOCTOC_topic_ID0ELZO1.html
#[derive(Serialize, Debug, Clone, PartialEq, Default)]
pub struct ToC {
    //  If no heading range is specified, all heading levels used in the document are listed.
    heading_styles_range: Option<(usize, usize)>,
}

impl ToC {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn heading_styles_range(mut self, start: usize, end: usize) -> Self {
        self.heading_styles_range = Some((start, end));
        self
    }
}

impl BuildXML for ToC {
    fn build(&self) -> Vec<u8> {
        let mut instr = "ToC".to_string();

        if let Some(heading_styles_range) = self.heading_styles_range {
            instr = format!(
                "{} \\o \"{}-{}\"",
                instr, heading_styles_range.0, heading_styles_range.1
            );
        }

        instr.into()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_field_character() {
        let b = ToC::new().heading_styles_range(1, 3).build();
        assert_eq!(str::from_utf8(&b).unwrap(), r#"ToC \o "1-3""#);
    }
}
