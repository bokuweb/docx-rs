use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;

#[derive(Debug, Clone)]
pub struct Indent {
    left: usize,
    special_indent: Option<SpecialIndentType>,
}

impl Indent {
    pub fn new(left: usize, special_indent: Option<SpecialIndentType>) -> Indent {
        Indent {
            left,
            special_indent,
        }
    }
}

impl BuildXML for Indent {
    fn build(&self) -> Vec<u8> {
        XMLBuilder::new()
            .indent(self.left, self.special_indent)
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
        let b = Indent::new(20, None).build();
        assert_eq!(str::from_utf8(&b).unwrap(), r#"<w:ind w:left="20" />"#);
    }

    #[test]
    fn test_first_line() {
        let b = Indent::new(20, Some(SpecialIndentType::FirstLine(40))).build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:ind w:left="20" w:firstLine="40" />"#
        );
    }

    #[test]
    fn test_hanging() {
        let b = Indent::new(20, Some(SpecialIndentType::Hanging(50))).build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:ind w:left="20" w:hanging="50" />"#
        );
    }
}
