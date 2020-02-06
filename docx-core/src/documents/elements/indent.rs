use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Indent {
    start: usize,
    end: Option<usize>,
    special_indent: Option<SpecialIndentType>,
}

impl Indent {
    pub fn new(
        start: usize,
        special_indent: Option<SpecialIndentType>,
        end: Option<usize>,
    ) -> Indent {
        Indent {
            start,
            end,
            special_indent,
        }
    }

    pub fn end(mut self, end: usize) -> Self {
        self.end = Some(end);
        self
    }
}

impl BuildXML for Indent {
    fn build(&self) -> Vec<u8> {
        XMLBuilder::new()
            .indent(
                self.start,
                self.special_indent,
                self.end.unwrap_or_default(),
            )
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
    fn test_left() {
        let b = Indent::new(20, None, None).build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:ind w:left="20" w:right="0" />"#
        );
    }

    #[test]
    fn test_first_line() {
        let b = Indent::new(20, Some(SpecialIndentType::FirstLine(40)), None).build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:ind w:left="20" w:right="0" w:firstLine="40" />"#
        );
    }

    #[test]
    fn test_hanging() {
        let b = Indent::new(20, Some(SpecialIndentType::Hanging(50)), None).build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:ind w:left="20" w:right="0" w:hanging="50" />"#
        );
    }
}
